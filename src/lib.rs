use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_yaml::Value as Yaml;
use std::collections::HashMap;
use std::fmt;

type Env = HashMap<String, Expr>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum Expr {
    Value(Value),
    Lambda(Box<Lambda>),
    IfElse(Box<IfElse>),
    LetIn(Box<LetIn>),
    Call(Call),
    Add(Add),
    Eq_(Eq_),
    Yaml(Val),
    Var(String),
}

impl From<Add> for Expr {
    fn from(v: Add) -> Self {
        Self::Add(v)
    }
}

impl From<Yaml> for Expr {
    fn from(v: Yaml) -> Self {
        Self::Yaml(Val::new(v))
    }
}

impl From<Val> for Expr {
    fn from(v: Val) -> Self {
        Self::Yaml(v)
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

impl From<Value> for Expr {
    fn from(v: Value) -> Self {
        Self::Value(v)
    }
}

impl Expr {
    pub fn eval(self, env: Env) -> Option<Value> {
        match self {
            Self::Yaml(v) => Some(Value::Yaml(v.yaml)),
            Self::Value(v) => Some(v),
            Self::Lambda(l) => {
                let func = Function {
                    args: l.lambda,
                    env,
                    expr: l.do_,
                };
                Some(Value::Function(Box::new(func)))
            }
            Self::Var(name) => match env.get(&name).cloned() {
                Some(Expr::Value(v)) => Some(v),
                Some(e) => e.eval(env),
                None => None,
            },

            Self::IfElse(cond) => {
                let res = cond.if_.eval(env.clone());
                match res {
                    Some(Value::Yaml(Yaml::Bool(true))) => cond.then.eval(env),
                    Some(Value::Yaml(Yaml::Bool(false))) => cond.else_.eval(env),
                    _ => None,
                }
            }

            Self::LetIn(letin) => {
                let mut env = env.clone();
                for (k, v) in &letin.let_ {
                    let val = v.clone();
                    env.insert(k.into(), val);
                }
                let res = letin.in_.eval(env);
                res
            }

            Self::Call(c) => {
                if let Some((func, args)) = c.args.split_first() {
                    if let Some(Value::Function(f)) = func.clone().eval(env) {
                        f.to_owned().call(args.clone().to_owned())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            Self::Add(s) => {
                let mut sum = Some(Value::Yaml(Yaml::Number(0.into())));
                for arg in s.args {
                    match (sum, arg.eval(env.clone())) {
                        (
                            Some(Value::Yaml(Yaml::Number(n1))),
                            Some(Value::Yaml(Yaml::Number(n2))),
                        ) => {
                            sum = Some(Value::Yaml(Yaml::Number(
                                (n1.as_i64().unwrap() + n2.as_i64().unwrap()).into(),
                            )));
                            // TODO: Handle all cases
                        }
                        (_, _) => sum = None, // Error
                    }
                }
                sum
            }

            Self::Eq_(e) => {
                if e.args.len() != 2 {
                    None
                } else {
                    let arg1 = e.args[0].clone().eval(env.clone());
                    let arg2 = e.args[1].clone().eval(env);
                    match (arg1, arg2) {
                        (Some(Value::Yaml(y1)), Some(Value::Yaml(y2))) => {
                            Some(Yaml::Bool(y1 == y2).into())
                        }
                        _ => Some(Yaml::Bool(false).into()),
                    }
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum Value {
    Yaml(Yaml),
    Function(Box<Function>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Yaml(y) => write!(f, "{:?}", y),
            Value::Function(fun) => write!(f, "<lambda: {}>", fun.args.join(" ")),
        }
    }
}

impl From<Yaml> for Value {
    fn from(y: Yaml) -> Self {
        Self::Yaml(y)
    }
}

impl From<Function> for Value {
    fn from(f: Function) -> Self {
        Self::Function(Box::new(f))
    }
}

impl Value {
    pub fn call<I>(self: Self, args: I) -> Option<Value>
    where
        I: IntoIterator<Item = Expr>,
    {
        match self {
            Value::Function(f) => f.call(args),
            _ => None,
        }
    }

    pub fn parse<T>(self: Self) -> Option<T>
    where
        T: DeserializeOwned,
    {
        if let Self::Yaml(y) = self {
            serde_yaml::from_value(y).ok()
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Function {
    args: Vec<String>,
    env: Env,
    expr: Expr,
}

impl Function {
    pub fn call<I>(mut self, args: I) -> Option<Value>
    where
        I: IntoIterator<Item = Expr>,
    {
        let mut args = args.into_iter();
        if let Some((name, rest)) = self.args.split_first() {
            if let Some(arg) = args.next() {
                self.env.insert(name.into(), arg);
                self.args = rest.to_vec();
                self.call(args)
            } else {
                Some(Value::Function(Box::new(self)))
            }
        } else {
            if args.count() == 0 {
                self.expr.eval(self.env)
            } else {
                None // TODO: Error
            }
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
pub struct Val {
    #[serde(rename = "$")]
    yaml: Yaml,
}

impl Val {
    pub fn new(yaml: Yaml) -> Self {
        Self { yaml }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Call {
    #[serde(rename = "()")]
    args: Vec<Expr>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Add {
    #[serde(rename = "+")]
    args: Vec<Expr>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Eq_ {
    #[serde(rename = "==")]
    args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Vm {
    env: Env,
}

impl Vm {
    pub fn new() -> Self {
        let env = HashMap::new();
        Self { env }
    }

    pub fn with_env<I>(mut self, env: I) -> Self
    where
        I: IntoIterator<Item = (String, Expr)>,
    {
        for (k, v) in env {
            self.env.insert(k, v);
        }
        self
    }

    pub fn eval(&self, expr: Expr) -> Option<Value> {
        expr.eval(self.env.clone())
    }
}
