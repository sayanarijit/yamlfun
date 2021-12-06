use yamlfun::platform::{DefaultPlatform, Platform};
use yamlfun::{yaml, Error, Expr, Function, Result, Value, Vm};

const PCALL: &str = r#"
:let:
  concept:
    :platform: import
    :arg: {:: ./concept.yml}
:in:
  [concept]
"#;

struct MyPlatform(DefaultPlatform);

impl Platform for MyPlatform {
    fn call(&self, name: &str, arg: Value) -> Result<Value> {
        match name {
            "import" => match arg {
                Value::String(s) => {
                    let yml = std::fs::read_to_string(s)
                        .map_err(|e| Error::PlatformCallError(e.to_string()))?;
                    let expr: Expr = yaml::from_str(&yml)?;
                    let func = Function::new([], expr);
                    Ok(Value::Function(func.into()))
                }
                v => Err(Error::InvalidArguments(name.into(), vec![v])),
            },
            _ => self.0.call(name, arg),
        }
    }
}

fn main() {
    let vm = Vm::new(MyPlatform(DefaultPlatform));

    let rec: Expr = yaml::from_str(PCALL.trim()).unwrap();
    let rec = vm.eval(rec).unwrap();
    println!("{}", &rec);
}
