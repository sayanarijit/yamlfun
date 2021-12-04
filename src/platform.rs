use crate::{Error, Result, Value};

pub trait Platform: Sized {
    fn call<I>(&self, name: &str, args: I) -> Result<Value>
    where
        I: IntoIterator<Item = Result<Value>>; // TODO: Error
}

pub struct NoPlatform;

impl Platform for NoPlatform {
    fn call<I>(&self, name: &str, _: I) -> Result<Value>
    where
        I: IntoIterator<Item = Result<Value>>,
    {
        Err(Error::PlatformCallError(format!(
            "cannot run {} in this platform",
            name
        )))
    }
}
