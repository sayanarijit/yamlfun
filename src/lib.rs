use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type Env = HashMap<String, Expr>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Add {
    #[serde(rename = "+")]
    args: Vec<Expr>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum Expr {
    Null,
    Bool(bool),
    Str(String),
    Var(Var),
    Number(i64),
    Float(f64),
    Lambda(Box<Lambda>),
    LetIn(Box<LetIn>),
    IfElse(Box<IfElse>),
}

// impl fmt::Display for Expr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Expr::Null => f.write_str("Null"),
//             Expr::Bool(v) => v.fmt(f),
//             Expr::Str(v) => {
//                 write!(f, "{:?}", v)
//             }
//             Expr::Number(v) => v.fmt(f),
//             Expr::Float(v) => v.fmt(f),
//             Expr::Var(v) => v.name.fmt(f),
//             Expr::Lambda(v) => {
//                 write!(f, "\\{:?} -> {}", &v.lambda, v.do_)?;
//                 Ok(())
//             }
//             Expr::LetIn(_) => todo!(),
//             Expr::IfElse(_) => todo!(),
//             Expr::Eval(_) => todo!(),
//         }
//     }
// }

impl Expr {
    pub fn call<I>(self, args: I) -> Option<Self>
    where
        I: IntoIterator<Item = Expr>,
    {
        match self {
            Self::Lambda(l) => l.as_caller().call(args),
            _ => None,
        }
    }

    pub fn sum(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => Some(Self::Number(n1 + n2)),
            (Self::Float(n1), Self::Float(n2)) => Some(Self::Float(n1 + n2)),
            (Self::Str(n1), Self::Str(n2)) => Some(Self::Str(format!("{}{}", n1, n2))),
            (_, _) => None,
        }
    }

    pub fn eval(mut self, env: &mut HashMap<String, Self>) -> Option<Self> {
        match self {
            Self::Null | Self::Bool(_) | Self::Number(_) | Self::Float(_) | Self::Str(_) => {
                Some(self)
            }

            Self::Lambda(ref mut l) => {
                for (k, v) in env {
                    if !l.env.contains_key(k) {
                        l.env.insert(k.into(), v.clone());
                    }
                }
                Some(self)
            }

            Self::Var(s) => env.get(&s.name).cloned().and_then(|v| v.eval(env)),

            Self::IfElse(cond) => match cond.if_.eval(env) {
                Some(Self::Bool(v)) => {
                    if v {
                        cond.then.eval(env)
                    } else {
                        cond.else_.eval(env)
                    }
                }
                _ => None,
            },

            Self::LetIn(letin) => {
                let mut past = HashMap::new();
                for (k, v) in &letin.let_ {
                    if let Some(v) = env.get(k) {
                        past.insert(k, v.clone());
                    }

                    let val = v.clone();
                    env.insert(k.into(), val);
                }

                let res = letin.in_.eval(env);

                for k in letin.let_.keys() {
                    env.remove(k);
                }

                for (k, v) in past {
                    env.insert(k.into(), v);
                }
                res
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Caller {
    args: Vec<String>,
    do_: Expr,
    env: Env,
}

impl Caller {
    fn call<I>(mut self, args: I) -> Option<Expr>
    where
        I: IntoIterator<Item = Expr>,
    {
        let mut args = args.into_iter();
        if let Some((name, rest)) = self.args.split_first() {
            if let Some(arg) = args.next() {
                if rest.is_empty() {
                    self.env.insert(name.to_string(), arg);
                    let letin = LetIn::new(self.env, self.do_);
                    Some(Expr::LetIn(Box::new(letin)))
                } else {
                    self.env.insert(name.to_string(), arg);
                    let lambda = Lambda::new(self.env.clone(), rest.to_vec(), self.do_);
                    Some(Expr::Lambda(Box::new(lambda)))
                }
            } else {
                let lambda = Lambda::new(self.env, self.args, self.do_);
                Some(Expr::Lambda(Box::new(lambda)))
            }
        } else {
            if let Some(_) = args.next() {
                None
            } else {
                let letin = LetIn::new(self.env, self.do_);
                Some(Expr::LetIn(Box::new(letin)))
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

    #[serde(default)]
    env: Env,
}

impl Lambda {
    pub fn new(env: Env, lambda: Vec<String>, do_: Expr) -> Self {
        Self { lambda, do_, env }
    }

    pub fn as_caller(self) -> Caller {
        Caller {
            args: self.lambda,
            do_: self.do_,
            env: self.env,
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

impl LetIn {
    pub fn new(let_: Env, in_: Expr) -> Self {
        Self { let_, in_ }
    }
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

impl IfElse {
    pub fn new(if_: Expr, then: Expr, else_: Expr) -> Self {
        Self { if_, then, else_ }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Var {
    #[serde(rename = "$")]
    name: String,
}

impl Var {
    pub fn new(name: String) -> Self {
        Self { name }
    }
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

    pub fn eval(&mut self, expr: Expr) -> Option<Expr> {
        expr.eval(&mut self.env)
    }

    pub fn call<I>(&mut self, expr: Expr, args: I) -> Option<Expr>
    where
        I: IntoIterator<Item = Expr>,
    {
        expr.call(args).unwrap().eval(&mut self.env)
    }
}
