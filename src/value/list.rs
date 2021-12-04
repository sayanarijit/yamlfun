use crate::Value;
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct List(pub(crate) Vec<Value>);

impl Deref for List {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Value>> for List {
    fn from(v: Vec<Value>) -> Self {
        Self(v)
    }
}

impl IntoIterator for List {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Serialize for List {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = s.serialize_seq(Some(self.0.len()))?;
        for e in self.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}
