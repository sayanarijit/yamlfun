use crate::platform::Platform;
use crate::value::{Function, Record as RecordVal};
use crate::{yaml, Env, Value, Yaml};
use crate::{Error, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum Expr {
    Call(Vec<Expr>),
    Lambda(Box<Lambda>),
    IfElse(Box<IfElse>),
    LetIn(Box<LetIn>),
    Sum(Sum),
    Equals(Equals),
    Constant(Constant),
    Variable(String),
    List(List),
    Record(Record),
    With(Box<With>),
    Get(Get),
    PlatformCall(Box<PlatformCall>),
    Chain(Box<Chain>),
    Value(#[serde(skip)] Value),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&yaml::to_string(self).unwrap())
    }
}

impl From<Value> for Expr {
    fn from(v: Value) -> Self {
        Self::Value(v)
    }
}

impl Default for Expr {
    fn default() -> Self {
        Self::Value(Value::Null)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Constant {
    #[serde(rename = ":")]
    yaml: Yaml,
}

impl From<Yaml> for Constant {
    fn from(yaml: Yaml) -> Self {
        Self { yaml }
    }
}

impl From<Box<LetIn>> for Expr {
    fn from(v: Box<LetIn>) -> Self {
        Self::LetIn(v)
    }
}

impl From<Box<IfElse>> for Expr {
    fn from(v: Box<IfElse>) -> Self {
        Self::IfElse(v)
    }
}

impl From<Box<Lambda>> for Expr {
    fn from(v: Box<Lambda>) -> Self {
        Self::Lambda(v)
    }
}

impl Expr {
    pub fn eval<P: Platform>(self, mut env: Env, platform: &P) -> Result<Value> {
        match self {
            Self::Value(v) => Ok(v),
            Self::Constant(y) => Ok(y.yaml.into()),
            Self::List(l) => {
                let mut items = vec![];
                for i in l.items {
                    let val = i.eval(env.clone(), platform)?;
                    items.push(val);
                }
                Ok(Value::List(items.into()))
            }

            Self::Record(r) => {
                let mut items = IndexMap::new();
                for (k, v) in r.items {
                    let val = v.eval(env.clone(), platform)?;
                    items.insert(k, val);
                }
                Ok(Value::Record(items.into()))
            }

            Self::Lambda(l) => {
                let func = Function {
                    args: l.lambda,
                    env,
                    expr: l.do_,
                };
                Ok(Value::Function(Box::new(func)))
            }

            Self::Variable(name) => {
                if let Some((first, rest)) = name.split_once('.') {
                    if let Some(e) = env.get(first) {
                        let val = e.clone().eval(env.clone(), platform)?;
                        val.get_from_yaml_nested(rest.split('.').map(RecordVal::de_field_name))
                            .map(Value::clone)
                    } else {
                        Err(Error::Undefined(first.into()))
                    }
                } else {
                    match env.get(&name).cloned() {
                        Some(Expr::Value(v)) => Ok(v),
                        Some(e) => e.eval(env, platform),
                        None => Err(Error::Undefined(name.clone())),
                    }
                }
            }

            Self::IfElse(cond) => {
                let res = cond.if_.eval(env.clone(), platform)?;
                match res {
                    Value::Bool(true) => cond.then.eval(env, platform),
                    Value::Bool(false) => cond.else_.eval(env, platform),
                    v => Err(Error::NotABoolean(v)),
                }
            }

            Self::LetIn(letin) => {
                for (k, v) in &letin.let_ {
                    let val = v.clone();
                    env.insert(k.into(), val);
                }
                let res = letin.in_.eval(env, platform);
                res
            }

            Self::Call(call) => {
                if let Some((func, args)) = call.split_first() {
                    let val = func.clone().eval(env.clone(), platform)?;
                    if let Value::Function(f) = val {
                        f.to_owned().call(
                            args.into_iter()
                                .map(|a| a.clone().eval(env.clone(), platform)),
                            platform,
                        )
                    } else {
                        Err(Error::NotAFunction(val))
                    }
                } else {
                    Err(Error::NoFunction)
                }
            }

            Self::Sum(s) => {
                let mut sum = Value::Number(0.into());
                for arg in s.args {
                    match (sum, arg.eval(env.clone(), platform)?) {
                        (Value::Number(n1), Value::Number(n2)) => {
                            sum =
                                Value::Number((n1.as_i64().unwrap() + n2.as_i64().unwrap()).into());
                            // TODO: Handle all cases
                        }
                        (n1, n2) => return Err(Error::InvalidArguments("+".into(), vec![n1, n2])),
                    }
                }
                Ok(sum)
            }

            Self::Equals(e) => {
                if e.args.len() != 2 {
                    Err(Error::NotEnoughArguments("+".into(), 2, e.args.len()))
                } else {
                    let arg1 = e.args[0].clone().eval(env.clone(), platform)?;
                    let arg2 = e.args[1].clone().eval(env, platform)?;
                    Ok(Yaml::Bool(arg1 == arg2).into())
                }
            }

            Self::With(w) => {
                for name in w.with {
                    match env.get(&name).cloned() {
                        Some(Self::Record(r)) => {
                            for (k, v) in r.items {
                                env.insert(k, v);
                            }
                        }

                        Some(Self::Constant(Constant { yaml })) => match yaml {
                            Yaml::Mapping(m) => {
                                for (k, v) in m {
                                    if let Yaml::String(s) = k {
                                        env.insert(s, Self::Constant(v.into()));
                                    }
                                }
                            }
                            _ => {}
                        },
                        Some(e) => return Err(Error::NotARecordExpr(e)),
                        None => return Err(Error::Undefined(name.into())),
                    }
                }
                w.do_.eval(env, platform)
            }

            Self::Get(g) => {
                if let Some((target, fields)) = g.args.split_first() {
                    let target = target.clone().eval(env.clone(), platform)?;
                    let fields_ = fields.into_iter().map(|f| {
                        let val = f.clone().eval(env.clone(), platform);
                        match val {
                            Ok(y) => yaml::to_value(y).map_err(From::from),
                            Err(e) => Err(e),
                        }
                    });

                    target.get_from_yaml_nested(fields_).map(Value::clone)
                } else {
                    Err(Error::NotEnoughArguments(".".into(), 2, g.args.len()))
                }
            }
            Expr::PlatformCall(p) => platform.call(&p.platform, p.arg.eval(env, platform)?),
            Self::Chain(c) => {
                if let Some((target, fields)) = c.args.split_first() {
                    let mut target = target.clone().eval(env.clone(), platform)?;
                    for f in fields {
                        if let Value::Function(f) = f.to_owned().eval(env.clone(), platform)? {
                            target = f.call([Ok(target)], platform)?;
                        }
                    }
                    Ok(target)
                } else {
                    Err(Error::NotEnoughArguments(".".into(), 2, c.args.len()))
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Lambda {
    lambda: Vec<String>,

    #[serde(rename = "do")]
    do_: Expr,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct With {
    with: Vec<String>,

    #[serde(rename = "do")]
    do_: Expr,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Get {
    #[serde(rename = ".")]
    args: Vec<Expr>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct PlatformCall {
    platform: String,
    arg: Expr,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Record {
    #[serde(rename = "rec")]
    items: IndexMap<String, Expr>,
}

impl Record {
    pub fn get_from_yaml(&self, field: &Yaml) -> Result<&Expr> {
        let field_ = RecordVal::ser_field_name(field);
        if let Some(val) = self.items.get(&field_) {
            Ok(val)
        } else {
            Err(Error::NotAField(field.clone()))
        }
    }
}

impl<I> From<I> for Record
where
    I: IntoIterator<Item = (String, Expr)>,
{
    fn from(items: I) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct List {
    #[serde(rename = "list")]
    items: Vec<Expr>,
}

impl<I> From<I> for List
where
    I: IntoIterator<Item = Expr>,
{
    fn from(items: I) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LetIn {
    #[serde(rename = "let")]
    let_: Env,

    #[serde(rename = "in")]
    in_: Expr,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct IfElse {
    #[serde(rename = "if")]
    if_: Expr,

    then: Expr,

    #[serde(rename = "else")]
    else_: Expr,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Sum {
    #[serde(rename = "+")]
    args: Vec<Expr>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Equals {
    #[serde(rename = "==")]
    args: Vec<Expr>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Chain {
    #[serde(rename = ":>")]
    args: Vec<Expr>,
}
