use yamlfun::platform::{DefaultPlatform, Platform};
use yamlfun::{vm, yaml, Error, Expr, Function, Result, Value, Vm, Env};

const PCALL: &str = r#"
:lambda: [path]
:do:
  - :platform: import
    :arg: path
"#;

struct MyPlatform(DefaultPlatform);

impl Platform for MyPlatform {
    fn init(&self, state: &mut vm::State) -> Result<()> {
        self.0.init(state)?;

        let import = yaml::from_str(PCALL)?;
        state.set_env("import".into(), import);
        Ok(())
    }

    fn call(&self, env: Env, name: &str, arg: Value) -> Result<Value> {
        match name {
            "import" => match arg {
                Value::String(s) => {
                    let yml = std::fs::read_to_string(s)
                        .map_err(|e| Error::PlatformCallError(e.to_string()))?;
                    let expr: Expr = yaml::from_str(&yml)?;
                    let func = Function::new(env, vec![], expr);
                    Ok(Value::Function(func.into()))
                }
                v => Err(Error::InvalidArguments(name.into(), vec![v])),
            },
            _ => self.0.call(env, name, arg),
        }
    }
}

fn main() {
    let platform = MyPlatform(DefaultPlatform);
    let vm = Vm::new(platform).unwrap();

    let rec: Expr = yaml::from_str("[import, {:: ./concept.yml}]").unwrap();
    let rec = vm.eval(rec).unwrap();
    println!("{}", &rec);
}
