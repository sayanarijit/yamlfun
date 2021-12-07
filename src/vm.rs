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

        // TODO: figure out how to split the lib.

        let std = include_str!("./Yaml/Std.yaml");
        let std: Expr = yaml::from_str(std)?;

        state.set_env("Std".into(), std);

        state.set_env("null_".into(), Expr::Variable("Std.null_".into()));
        state.set_env("Bool".into(), Expr::Variable("Std.Bool".into()));
        state.set_env("true_".into(), Expr::Variable("Std.Bool.true_".into()));
        state.set_env("false_".into(), Expr::Variable("Std.Bool.false_".into()));
        state.set_env("add".into(), Expr::Variable("Std.add".into()));
        state.set_env("List".into(), Expr::Variable("Std.List".into()));
        state.set_env("Maybe".into(), Expr::Variable("Std.Maybe".into()));
        state.set_env("add".into(), Expr::Variable("Std.add".into()));
        state.set_env("(+)".into(), Expr::Variable("Std.(+)".into()));
        state.set_env("not".into(), Expr::Variable("Std.not".into()));
        state.set_env("(!)".into(), Expr::Variable("Std.(!)".into()));
        state.set_env("eq".into(), Expr::Variable("Std.(eq)".into()));
        state.set_env("ne".into(), Expr::Variable("Std.(ne)".into()));
        state.set_env("!=".into(), Expr::Variable("Std.(!=)".into()));
        state.set_env("composeL".into(), Expr::Variable("Std.composeL".into()));
        state.set_env("(<<)".into(), Expr::Variable("Std.(<<)".into()));
        state.set_env("composeR".into(), Expr::Variable("Std.composeR".into()));
        state.set_env("(>>)".into(), Expr::Variable("Std.(>>)".into()));
        state.set_env("and".into(), Expr::Variable("Std.and".into()));
        state.set_env("(&&)".into(), Expr::Variable("Std.(&&)".into()));
        state.set_env("or".into(), Expr::Variable("Std.or".into()));
        state.set_env("(||)".into(), Expr::Variable("Std.(||)".into()));
        state.set_env("xor".into(), Expr::Variable("Std.xor".into()));
        state.set_env("Maybe".into(), Expr::Variable("Std.Maybe".into()));
        state.set_env("List".into(), Expr::Variable("Std.List".into()));
        state.set_env("Rec".into(), Expr::Variable("Std.Rec".into()));

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
