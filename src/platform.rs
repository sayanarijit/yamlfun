use crate::{vm, Error, Result, Value};

pub trait Platform: Sized {
    fn init(&self, state: &mut vm::State) -> Result<()>;
    fn call(&self, name: &str, arg: Value) -> Result<Value>;
}

#[derive(Default, Debug)]
pub struct DefaultPlatform;

impl Platform for DefaultPlatform {
    fn init(&self, _state: &mut vm::State) -> Result<()> {
        Ok(())
    }

    fn call(&self, name: &str, _: Value) -> Result<Value> {
        Err(Error::PlatformCallError(format!(
            "cannot call {} in this platform",
            name
        )))
    }
}
