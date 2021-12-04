use crate::platform::Platform;
use crate::{Env, Error, Expr, Result, Value};

pub struct Vm<P: Platform> {
    env: Env,
    platform: P,
}

impl<P: Platform> Vm<P> {
    pub fn new(platform: P) -> Self
    where
        P: Platform,
    {
        Self {
            platform,
            env: Default::default(),
        }
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

    pub fn eval(&self, expr: Expr) -> Result<Value> {
        expr.eval(self.env.clone(), &self.platform)
    }

    pub fn call<I>(&self, func: Value, args: I) -> Result<Value>
    where
        I: IntoIterator<Item = Expr>,
    {
        match func {
            Value::Function(func) => {
                let args = args
                    .into_iter()
                    .map(|a| a.eval(self.env.clone(), &self.platform));
                func.call(args, &self.platform)
            }
            _ => Err(Error::NotAFunction(func)),
        }
    }
}
