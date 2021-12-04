use crate::{yaml, yaml::Value as Yaml, Value};
use crate::{Error, Result};
use serde::ser::{Serialize, SerializeMap, Serializer};
use std::collections::HashMap;
use std::ops::Deref;
use std::result;

#[derive(Debug, Clone, PartialEq)]
pub struct Record(HashMap<String, Value>);

impl Deref for Record {
    type Target = HashMap<String, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HashMap<String, Value>> for Record {
    fn from(m: HashMap<String, Value>) -> Self {
        Self(m)
    }
}

impl IntoIterator for Record {
    type Item = (String, Value);
    type IntoIter = std::collections::hash_map::IntoIter<String, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Serialize for Record {
    fn serialize<S>(&self, s: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = s.serialize_map(Some(self.0.len()))?;
        for (k, v) in self.iter() {
            let y = Self::de_field_name(k).unwrap(); // TODO: Conv Error
            map.serialize_entry(&y, v)?;
        }
        map.end()
    }
}

impl Record {
    pub fn ser_field_name(y: &Yaml) -> String {
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

    pub fn de_field_name(field: &str) -> Result<Yaml> {
        if field.starts_with("((") && field.ends_with("))") {
            let y = Self::de_field_name(
                field
                    .strip_prefix("((")
                    .unwrap()
                    .strip_suffix("))")
                    .unwrap(),
            )?;
            Ok(y)
        } else if field.starts_with('(') && field.ends_with(')') {
            let y = yaml::from_str(field.strip_prefix('(').unwrap().strip_suffix(')').unwrap())?;
            Ok(y)
        } else {
            Ok(Yaml::String(field.into()))
        }
    }

    pub fn get_from_yaml(&self, field: &Yaml) -> Result<&Value> {
        let field_ = Self::ser_field_name(field);
        if let Some(val) = self.get(&field_) {
            Ok(val)
        } else {
            Err(Error::NotAField(field.clone()))
        }
    }
}
