use crate::platform::Platform;
use crate::{Env, Error, Expr, Result, Value};

#[derive(Default)]
pub struct State {
    env: Env,
}

impl State {
    pub fn set_env(&mut self, name: String, expr: Expr) {
        self.env.insert(name, expr);
    }
}

pub struct Vm<P: Platform> {
    state: State,
    platform: P,
}

impl<P: Platform> Vm<P> {
    pub fn new(platform: P) -> Result<Self>
    where
        P: Platform,
    {
        let mut state: State = Default::default();
        platform.init(&mut state)?;

        Ok(Self { platform, state })
    }

    pub fn with_env<I>(mut self, env: I) -> Self
    where
        I: IntoIterator<Item = (String, Expr)>,
    {
        for (k, v) in env {
            self.set_env(k, v);
        }
        self
    }

    pub fn set_env(&mut self, name: String, expr: Expr) {
        self.state.set_env(name, expr);
    }

    pub fn eval(&self, expr: Expr) -> Result<Value> {
        expr.eval(self.state.env.clone(), &self.platform)
    }

    pub fn call<I>(&self, func: Value, args: I) -> Result<Value>
    where
        I: IntoIterator<Item = Expr>,
    {
        match func {
            Value::Function(func) => {
                let args = args
                    .into_iter()
                    .map(|a| a.eval(self.state.env.clone(), &self.platform));
                func.call(args, &self.platform)
            }
            _ => Err(Error::NotAFunction(func)),
        }
    }
}
