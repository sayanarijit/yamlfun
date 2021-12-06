use yamlfun::{yaml, DefaultPlatform, Expr, Value, Vm};

const ONE: &str = r#"
:lambda: []
:do: {:: 1}
"#;

const TWO: &str = r#"
:lambda: []
:do:
  :lambda: []
  :do: {:: 2}
"#;

const SUM: &str = r#"
:lambda: [arg2, arg1]
:do:
  :+: [arg1, arg2]
"#;

fn main() {
    let vm = Vm::new(DefaultPlatform);

    let one: Expr = yaml::from_str(ONE.trim()).unwrap();
    let one = vm.eval(one.clone()).unwrap();
    let one = vm.call(one, []).unwrap();
    println!("{}", &one);

    let two: Expr = yaml::from_str(TWO.trim()).unwrap();
    let two = vm.eval(two.clone()).unwrap();
    let two = vm.call(two, []).unwrap();
    let two = vm.call(two, []).unwrap();
    println!("{}", two);

    let args: Expr = yaml::from_str(SUM.trim()).unwrap();
    let args = vm.eval(args.clone()).unwrap();
    println!("{}", &args);

    let args = vm.call(args, []).unwrap();
    println!("{}", &args);

    let args = vm.call(args, [Value::Number(10.into()).into()]).unwrap();
    println!("{}", &args);

    let args = vm.call(args, [Value::Number(30.into()).into()]).unwrap();
    println!("{}", &args);

    let args: Expr = yaml::from_str(SUM.trim()).unwrap();
    let args = vm.eval(args.clone()).unwrap();
    let args = vm
        .call(
            args,
            [
                Value::Number(1.into()).into(),
                Value::Number(101.into()).into(),
            ],
        )
        .unwrap();
    println!("{}", &args);
}
