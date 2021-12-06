use crate::platform::Platform;
use crate::Env;
use crate::Expr;
use crate::Value;
use crate::{Error, Result as CrateResult};
use serde::ser::{Error as SerdeError, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Function {
    pub(crate) env: Env,
    pub(crate) args: Vec<String>,
    pub(crate) expr: Expr,
}

impl Function {
    pub fn new<I>(env: Env, args: I, expr: Expr) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        Self {
            env,
            args: args.into_iter().collect(),
            expr,
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

    pub fn call<I, P>(mut self, args: I, platform: &P) -> CrateResult<Value>
    where
        I: IntoIterator<Item = CrateResult<Value>>,
        P: Platform,
    {
        let mut args = args.into_iter();
        if let Some((name, rest)) = self.args.split_first() {
            if let Some(arg) = args.next() {
                self.env.insert(name.into(), arg?.into());
                self.args = rest.to_vec();
                self.call(args, platform)
            } else {
                Ok(Value::Function(Box::new(self)))
            }
        } else {
            if args.count() == 0 {
                self.expr.eval(self.env, platform)
            } else {
                Err(Error::Undefined("".into()))
            }
        }
    }
}

impl Serialize for Function {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Err(SerdeError::custom("cannot serialize function"))
    }
}
