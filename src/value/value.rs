use crate::value::{Function, List, Number, Record};
use crate::yaml;
use crate::yaml::Value as Yaml;
use crate::{Error, Result};
use indexmap::IndexMap;
use serde::ser::{Serialize, Serializer};
use serde::Deserialize;
use std::convert::TryInto;
use std::fmt;
use std::result;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(from = "Yaml")]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    List(List),
    Record(Record),
    Function(Box<Function>),
}

impl From<Box<Function>> for Value {
    fn from(v: Box<Function>) -> Self {
        Self::Function(v)
    }
}

impl From<Record> for Value {
    fn from(v: Record) -> Self {
        Self::Record(v)
    }
}

impl From<List> for Value {
    fn from(v: List) -> Self {
        Self::List(v)
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

impl From<IndexMap<String, Value>> for Value {
    fn from(v: IndexMap<String, Value>) -> Self {
        Self::Record(v.into())
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Self::List(v.into())
    }
}

impl From<Function> for Value {
    fn from(v: Function) -> Self {
        Self::Function(Box::new(v))
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
            Value::Null => write!(f, "null"),
            Value::Function(fun) => write!(f, "ƒ({})", fun.args.join(", ")),
            Value::Bool(b) => b.fmt(f),
            Value::Number(n) => n.fmt(f),
            Value::String(s) => write!(f, "{:?}", s),
            Value::List(l) => {
                let len = l.0.len();
                write!(f, "[")?;
                for (i, e) in l.0.iter().enumerate() {
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

impl Serialize for Value {
    fn serialize<S>(&self, s: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Null => s.serialize_unit(),
            Value::Bool(v) => s.serialize_bool(*v),
            Value::Number(v) => v.serialize(s),
            Value::String(v) => s.serialize_str(v),
            Value::List(v) => v.serialize(s),
            Value::Record(v) => v.serialize(s),
            Value::Function(v) => v.serialize(s),
        }
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
                    .map(|(k, v)| (Record::ser_field_name(&k), v.into()))
                    .collect::<IndexMap<String, Value>>()
                    .into(),
            ),
        }
    }
}

impl TryInto<Yaml> for Value {
    type Error = Error;

    fn try_into(self) -> Result<Yaml> {
        let y = yaml::to_value(self)?;
        Ok(y)
    }
}

impl Value {
    pub fn get_from_yaml_nested<I>(&self, fields: I) -> Result<&Value>
    where
        I: IntoIterator<Item = Result<Yaml>>,
    {
        let mut val = self;
        for f in fields {
            match val {
                Self::Record(r) => {
                    val = r.get_from_yaml(&f?)?;
                }
                _ => return Err(Error::NotARecord(self.clone())),
            }
        }
        Ok(val)
    }
}
