use crate::{Error, Result, Value};

pub trait Platform: Sized {
    fn call(&self, name: &str, arg: Value) -> Result<Value>;
}

#[derive(Default, Debug)]
pub struct DefaultPlatform;

impl Platform for DefaultPlatform {
    fn call(&self, name: &str, _: Value) -> Result<Value> {
        Err(Error::PlatformCallError(format!(
            "cannot call {} in this platform",
            name
        )))
    }
}
