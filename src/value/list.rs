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

impl<I> From<I> for List
where
    I: IntoIterator<Item = Value>,
{
    fn from(items: I) -> Self {
        Self(items.into_iter().collect())
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
