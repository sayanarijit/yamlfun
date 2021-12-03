use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_yaml::from_value;
use serde_yaml::Mapping;
use serde_yaml::Number;
use serde_yaml::Value as Yaml;
use std::collections::HashMap;
use std::fmt;

type Env = HashMap<String, Expr>;

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
    Value(#[serde(skip)] Value),
}

impl Default for Expr {
    fn default() -> Self {
        Self::Value(Value::Null)
    }
}

impl From<Sum> for Expr {
    fn from(v: Sum) -> Self {
        Self::Sum(v)
    }
}

impl From<Equals> for Expr {
    fn from(v: Equals) -> Self {
        Self::Equals(v)
    }
}

impl From<Constant> for Expr {
    fn from(v: Constant) -> Self {
        Self::Constant(v)
    }
}

impl From<String> for Expr {
    fn from(v: String) -> Self {
        Self::Variable(v)
    }
}

impl From<Value> for Expr {
    fn from(v: Value) -> Self {
        Self::Value(v)
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
    pub fn eval(self, mut env: Env) -> Option<Value> {
        match self {
            Self::Value(v) => Some(v),
            Self::Constant(y) => Some(y.yaml.into()),
            Self::List(l) => {
                let mut items = vec![];
                for i in l.items {
                    if let Some(val) = i.eval(env.clone()) {
                        items.push(val);
                    } else {
                        return None;
                    }
                }
                Some(Value::List(items))
            }

            Self::Record(r) => {
                let mut items = HashMap::new();
                for (k, v) in r.items {
                    if let Some(val) = v.eval(env.clone()) {
                        items.insert(k, val);
                    } else {
                        return None;
                    }
                }
                Some(Value::Record(items))
            }

            Self::Lambda(l) => {
                let func = Function {
                    args: l.lambda,
                    env,
                    expr: l.do_,
                };
                Some(Value::Function(Box::new(func)))
            }

            Self::Variable(name) => {
                if let Some((first, rest)) = name.split_once('.') {
                    env.get(first)
                        .and_then(|e| {
                            e.clone()
                                .get(&rest.split('.').map(str::to_string).collect::<Vec<String>>())
                        })
                        .and_then(|e| e.eval(env))
                } else {
                    match env.get(&name).cloned() {
                        Some(Expr::Value(v)) => Some(v),
                        Some(e) => e.eval(env),
                        None => None,
                    }
                }
            }

            Self::IfElse(cond) => {
                let res = cond.if_.eval(env.clone());
                match res {
                    Some(Value::Bool(true)) => cond.then.eval(env),
                    Some(Value::Bool(false)) => cond.else_.eval(env),
                    _ => None,
                }
            }

            Self::LetIn(letin) => {
                for (k, v) in &letin.let_ {
                    let val = v.clone();
                    env.insert(k.into(), val);
                }
                let res = letin.in_.eval(env);
                res
            }

            Self::Call(call) => {
                if let Some((func, args)) = call.split_first() {
                    if let Some(Value::Function(f)) = func.clone().eval(env) {
                        f.to_owned().call(args.clone().to_owned())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            Self::Sum(s) => {
                let mut sum = Some(Value::Number(0.into()));
                for arg in s.args {
                    match (sum, arg.eval(env.clone())) {
                        (Some(Value::Number(n1)), Some(Value::Number(n2))) => {
                            sum = Some(Value::Number(
                                (n1.as_i64().unwrap() + n2.as_i64().unwrap()).into(),
                            ));
                            // TODO: Handle all cases
                        }
                        (_, _) => sum = None, // Error
                    }
                }
                sum
            }

            Self::Equals(e) => {
                if e.args.len() != 2 {
                    None
                } else {
                    let arg1 = e.args[0].clone().eval(env.clone());
                    let arg2 = e.args[1].clone().eval(env);
                    match (arg1, arg2) {
                        (Some(v1), Some(v2)) => Some(Yaml::Bool(v1 == v2).into()),
                        _ => Some(Yaml::Bool(false).into()),
                    }
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
                        _ => {}
                    }
                }
                w.do_.eval(env)
            }

            Self::Get(g) => {
                if let Some((target, fields)) = g.args.split_first() {
                    let mut val = target.clone().eval(env.clone());
                    for field in fields {
                        let field = field.clone().eval(env.clone());
                        match (val, field) {
                            (Some(Value::Record(r)), Some(v)) => {
                                val = v
                                    .to_yaml()
                                    .and_then(|y| r.get(to_key(&y).as_str()))
                                    .cloned();
                            }
                            _ => {
                                return None;
                            }
                        }
                    }
                    val
                } else {
                    None
                }
            }
        }
    }

    pub fn get(self, fields: &[String]) -> Option<Expr> {
        if let Some((first, rest)) = fields.split_first() {
            match self {
                Self::Record(r) => r.items.get(first).cloned().and_then(|e| e.get(rest)),
                Self::Constant(c) => match c.yaml {
                    Yaml::Mapping(ref m) => m
                        .get(&from_key(first))
                        .and_then(|v| Self::Constant(v.clone().into()).get(rest)),
                    _ => None,
                },
                _ => None,
            }
        } else {
            Some(self.clone())
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
pub struct Record {
    #[serde(rename = "rec")]
    items: HashMap<String, Expr>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    List(Vec<Value>),
    Record(HashMap<String, Value>),
    Function(Box<Function>),
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}
impl From<HashMap<String, Value>> for Value {
    fn from(v: HashMap<String, Value>) -> Self {
        Self::Record(v)
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Self::List(v)
    }
}

impl From<Box<Function>> for Value {
    fn from(v: Box<Function>) -> Self {
        Self::Function(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<Number> for Value {
    fn from(v: Number) -> Self {
        Self::Number(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Function(fun) => write!(f, "ƒ({})", fun.args.join(", ")),
            Value::Bool(b) => b.fmt(f),
            Value::Number(n) => n.fmt(f),
            Value::String(s) => write!(f, "{:?}", s),
            Value::List(l) => {
                let len = l.len();
                write!(f, "[")?;
                for (i, e) in l.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i + 1 != len {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Value::Record(r) => {
                let len = r.len();
                write!(f, "{{")?;
                for (i, (k, v)) in r.iter().enumerate() {
                    write!(f, "{}: {}", k, v)?;
                    if i + 1 != len {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
        }
    }
}

fn to_key(y: &Yaml) -> String {
    match y {
        Yaml::Null => "(null)".into(),
        Yaml::String(s) => {
            if s.starts_with('(') && s.ends_with(')') {
                format!("({})", s)
            } else {
                s.into()
            }
        }
        Yaml::Bool(b) => format!("({})", b),
        Yaml::Number(n) => format!("({})", n),
        Yaml::Sequence(s) => format!("({:?})", s),
        Yaml::Mapping(m) => format!("({:?})", m),
    }
}

fn from_key(key: &str) -> Yaml {
    if key.starts_with("((") && key.ends_with("))") {
        from_key(key.strip_prefix("((").unwrap().strip_suffix("))").unwrap())
    } else if key.starts_with('(') && key.ends_with(')') {
        serde_yaml::from_str(key.strip_prefix('(').unwrap().strip_suffix(')').unwrap()).unwrap()
    } else {
        Yaml::String(key.into())
    }
}

impl From<Yaml> for Value {
    fn from(y: Yaml) -> Self {
        match y {
            Yaml::Null => Self::Null,
            Yaml::Bool(bool) => Self::Bool(bool),
            Yaml::Number(n) => Self::Number(n),
            Yaml::String(s) => Self::String(s),
            Yaml::Sequence(s) => Self::List(
                s.into_iter()
                    .map(Value::from)
                    .collect::<Vec<Value>>()
                    .into(),
            ),
            Yaml::Mapping(m) => Self::Record(
                m.into_iter()
                    .map(|(k, v)| (to_key(&k), v.into()))
                    .collect::<HashMap<String, Value>>()
                    .into(),
            ),
        }
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

    pub fn to_yaml(self: Self) -> Option<Yaml> {
        match self {
            Self::Null => Some(Yaml::Null),
            Self::Bool(b) => Some(Yaml::Bool(b)),
            Value::Number(n) => Some(Yaml::Number(n)),
            Value::String(s) => Some(Yaml::String(s)),
            Value::List(l) => Some(Yaml::Sequence(
                l.into_iter().filter_map(Value::to_yaml).collect(),
            )),
            Value::Record(r) => Some(Yaml::Mapping(
                r.into_iter()
                    .filter_map(|(k, v)| v.to_yaml().map(|val| (Yaml::String(k), val)))
                    .collect::<Mapping>(),
            )),
            Value::Function(_) => None,
        }
    }

    pub fn parse<T>(self: Self) -> Option<T>
    where
        T: DeserializeOwned,
    {
        self.to_yaml().and_then(|v| from_value(v).ok())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
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
