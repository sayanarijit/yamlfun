use crate::{vm, Env, Error, Result, Value};

pub trait Platform: Sized {
    fn init(&self, state: &mut vm::State) -> Result<()>;
    fn call(&self, env: Env, name: &str, arg: Value) -> Result<Value>;
}

#[derive(Default, Debug)]
pub struct DefaultPlatform;

impl Platform for DefaultPlatform {
    fn init(&self, _state: &mut vm::State) -> Result<()> {
        Ok(())
    }

    fn call(&self, _env: Env, name: &str, _: Value) -> Result<Value> {
        Err(Error::PlatformCallError(format!(
            "cannot call {} in this platform",
            name
        )))
    }
}
