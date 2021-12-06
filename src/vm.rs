use crate::platform::Platform;
use crate::{yaml, Env, Error, Expr, Result, Value};

#[derive(Default, Debug, PartialEq)]
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

        let basics = include_str!("./Std/Basics.yaml");
        let basics: Expr = yaml::from_str(basics)?;

        state.set_env("Basics".into(), basics);
        state.set_env("(+)".into(), Expr::Variable("Basics.(+)".into()));
        state.set_env("(!)".into(), Expr::Variable("Basics.(!)".into()));
        state.set_env("(==)".into(), Expr::Variable("Basics.(==)".into()));
        state.set_env("(!=)".into(), Expr::Variable("Basics.(!=)".into()));
        state.set_env("(<<)".into(), Expr::Variable("Basics.(<<)".into()));
        state.set_env("(>>)".into(), Expr::Variable("Basics.(>>)".into()));
        state.set_env("(&&)".into(), Expr::Variable("Basics.(&&)".into()));
        state.set_env("(||)".into(), Expr::Variable("Basics.(||)".into()));
        state.set_env("xor".into(), Expr::Variable("Basics.xor".into()));
        state.set_env("(++)".into(), Expr::Variable("Basics.(++)".into()));
        state.set_env("cons".into(), Expr::Variable("Basics.cons".into()));

        let list = include_str!("./Std/List.yaml");
        let list: Expr = yaml::from_str(list)?;
        state.set_env("List".into(), list);

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
