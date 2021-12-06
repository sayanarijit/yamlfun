use crate::platform::Platform;
use crate::value::{Function, Record as RecordVal};
use crate::{yaml, Env, Value, Yaml};
use crate::{Error, Result};
use indexmap::IndexMap;
use indexmap::IndexSet;
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
    List(Box<List>),
    Record(Record),
    With(Box<With>),
    Get(Get),
    Update(Box<Update>),
    PlatformCall(Box<PlatformCall>),
    Chain(Box<Chain>),
    CaseOf(Box<CaseOf>),
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

            Self::Lambda(l) => Ok(Value::Function(Box::new(l.to_function(env)))),

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
                let mut args = s.args.into_iter();
                if let Some(sum) = args.next().map(|a| a.eval(env.clone(), platform)) {
                    let mut sum = sum?;
                    for arg in args {
                        let arg = arg.eval(env.clone(), platform)?;
                        match (&sum, &arg) {
                            (Value::Number(n1), Value::Number(n2)) => {
                                if let Some(s) = n1
                                    .as_f64()
                                    .and_then(|i1| n2.as_f64().map(|i2| (i1 + i2).into()))
                                    .or_else(|| {
                                        n1.as_u64()
                                            .and_then(|i1| n2.as_u64().map(|i2| (i1 + i2).into()))
                                    })
                                    .or_else(|| {
                                        n1.as_i64()
                                            .and_then(|i1| n2.as_i64().map(|i2| (i1 + i2).into()))
                                    })
                                {
                                    sum = Value::Number(s);
                                } else {
                                    return Err(Error::InvalidArguments(
                                        "+".into(),
                                        vec![sum, arg],
                                    ));
                                }
                            }
                            (Value::List(l1), Value::List(l2)) => {
                                let mut list = l1.0.clone();
                                list.append(&mut l2.0.clone());
                                sum = Value::List(list.into());
                            }
                            (Value::String(s1), Value::String(s2)) => {
                                sum = Value::String(format!("{}{}", s1, s2));
                            }
                            (n1, n2) => {
                                return Err(Error::InvalidArguments(
                                    "+".into(),
                                    vec![n1.clone(), n2.clone()],
                                ))
                            }
                        }
                    }
                    Ok(sum)
                } else {
                    Err(Error::NotEnoughArguments("+".into(), 1, 0))
                }
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

            Expr::Update(u) => {
                let rec = u.update.clone().eval(env.clone(), platform)?;

                match rec {
                    Value::Record(r) => {
                        let mut newrec: IndexMap<String, Value> = r
                            .iter()
                            .filter(|(k, _)| !u.unset.contains(*k))
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect();

                        for (k, v) in u.set {
                            let val = v.eval(env.clone(), platform)?;
                            newrec.insert(k, val);
                        }

                        Ok(newrec.into())
                    }
                    _ => Err(Error::NotARecord(rec)),
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

            Expr::CaseOf(c) => {
                let case = c.case.eval(env.clone(), platform)?;

                if let Ok(y) = yaml::to_value(&case) {
                    if let Some(e) = c.of.exact.get(&y).cloned() {
                        return e.eval(env, platform);
                    }
                }

                match &case {
                    Value::Null => {
                        c.of.unit
                            .map(|u| u.eval(env, platform))
                            .unwrap_or_else(|| Err(Error::CaseError(case.into())))
                    }

                    Value::Bool(b) => {
                        c.of.boolean
                            .map(|l| {
                                let args = match l.lambda.len() {
                                    0 => vec![],
                                    1 => vec![Ok(b.clone().into())],
                                    _ => return Err(Error::CaseError(b.clone().into())),
                                };
                                let func = l.to_function(env);
                                func.call(args, platform)
                            })
                            .unwrap_or_else(|| Err(Error::CaseError(b.clone().into())))
                    }

                    Value::Number(n) => {
                        if n.is_i64() || n.is_u64() {
                            c.of.integer
                                .map(|l| {
                                    let args = match l.lambda.len() {
                                        0 => vec![],
                                        1 => vec![Ok(n.clone().into())],
                                        _ => return Err(Error::CaseError(n.clone().into())),
                                    };
                                    let func = l.to_function(env);
                                    func.call(args, platform)
                                })
                                .unwrap_or_else(|| Err(Error::CaseError(n.clone().into())))
                        } else {
                            c.of.float
                                .map(|l| {
                                    let args = match l.lambda.len() {
                                        0 => vec![],
                                        1 => vec![Ok(case.clone().into())],
                                        _ => return Err(Error::CaseError(case.clone().into())),
                                    };
                                    let func = l.to_function(env);
                                    func.call(args, platform)
                                })
                                .unwrap_or_else(|| Err(Error::CaseError(case.clone().into())))
                        }
                    }

                    Value::String(s) => c
                        .of
                        .string
                        .map(|l| {
                            let args = match l.lambda.len() {
                                0 => vec![],
                                1 => vec![Ok(s.clone().into())],
                                2 => {
                                    let (head, tail) = s.split_at(1);
                                    vec![Ok(head.to_string().into()), Ok(tail.to_string().into())]
                                }
                                _ => return Err(Error::CaseError(s.clone().into())),
                            };
                            let func = l.to_function(env);
                            func.call(args, platform)
                        })
                        .unwrap_or_else(|| Err(Error::CaseError(s.clone().into()))),

                    Value::List(s) => {
                        c.of.list
                            .map(|l| {
                                let args = match l.lambda.len() {
                                    0 => vec![],
                                    1 => vec![Ok(case.clone().into())],
                                    2 => {
                                        if let Some((head, tail)) = s.split_first() {
                                            vec![
                                                Ok(head.clone().into()),
                                                Ok(Value::List(
                                                    tail.clone()
                                                        .into_iter()
                                                        .map(Clone::clone)
                                                        .map(Value::from)
                                                        .into(),
                                                )),
                                            ]
                                        } else {
                                            vec![Ok(Value::Null), Ok(Value::List(vec![].into()))]
                                        }
                                    }
                                    _ => return Err(Error::CaseError(case.clone().into())),
                                };
                                let func = l.to_function(env);
                                func.call(args, platform)
                            })
                            .unwrap_or_else(|| Err(Error::CaseError(case.clone().into())))
                    }

                    Value::Record(_) => {
                        c.of.rec
                            .map(|l| {
                                let args = match l.lambda.len() {
                                    0 => vec![],
                                    1 => vec![Ok(case.clone().into())],
                                    _ => return Err(Error::CaseError(case.clone().into())),
                                };
                                let func = l.to_function(env);
                                func.call(args, platform)
                            })
                            .unwrap_or_else(|| Err(Error::CaseError(case.clone().into())))
                    }

                    Value::Function(f) => {
                        c.of.function
                            .map(|l| {
                                let args = match l.lambda.len() {
                                    0 => vec![],
                                    1 => vec![Ok(f.clone().into())],
                                    _ => return Err(Error::CaseError(f.clone().into())),
                                };
                                let func = l.to_function(env);
                                func.call(args, platform)
                            })
                            .unwrap_or_else(|| Err(Error::CaseError(f.clone().into())))
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Lambda {
    #[serde(rename = ":lambda")]
    lambda: Vec<String>,

    #[serde(rename = ":do")]
    do_: Expr,
}

impl Lambda {
    fn to_function(self, env: Env) -> Function {
        Function {
            args: self.lambda,
            env,
            expr: self.do_,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct With {
    #[serde(rename = ":with")]
    with: Vec<String>,

    #[serde(rename = ":do")]
    do_: Expr,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Get {
    #[serde(rename = ":.")]
    args: Vec<Expr>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Update {
    #[serde(rename = ":update")]
    update: Expr,

    #[serde(default)]
    #[serde(rename = ":set")]
    set: IndexMap<String, Expr>,

    #[serde(default)]
    #[serde(rename = ":unset")]
    unset: IndexSet<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct PlatformCall {
    #[serde(rename = ":platform")]
    platform: String,
    #[serde(rename = ":arg")]
    arg: Expr,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct Record {
    #[serde(rename = ":rec")]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(deny_unknown_fields)]
pub struct List {
    #[serde(rename = ":list")]
    items: Vec<Expr>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LetIn {
    #[serde(rename = ":let")]
    let_: Env,

    #[serde(rename = ":in")]
    in_: Expr,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct IfElse {
    #[serde(rename = ":if")]
    if_: Expr,

    #[serde(rename = ":then")]
    then: Expr,

    #[serde(rename = ":else")]
    else_: Expr,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Sum {
    #[serde(rename = ":+")]
    args: Vec<Expr>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Equals {
    #[serde(rename = ":==")]
    args: Vec<Expr>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Chain {
    #[serde(rename = ":|>")]
    args: Vec<Expr>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CaseOf {
    #[serde(rename = ":case")]
    case: Expr,
    #[serde(rename = ":of")]
    of: Matcher,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Matcher {
    #[serde(default, rename = ":==")]
    exact: IndexMap<Yaml, Expr>,

    #[serde(default, rename = ":()")]
    unit: Option<Expr>,

    #[serde(default, rename = ":bool")]
    boolean: Option<Lambda>,

    #[serde(default, rename = ":int")]
    integer: Option<Lambda>,

    #[serde(default, rename = ":float")]
    float: Option<Lambda>,

    #[serde(default, rename = ":string")]
    string: Option<Lambda>,

    #[serde(default, rename = ":function")]
    function: Option<Lambda>,

    #[serde(default, rename = ":list")]
    list: Option<Lambda>,

    #[serde(default, rename = ":rec")]
    rec: Option<Lambda>,

    #[serde(default, rename = ":_")]
    default: Option<Lambda>,
}
