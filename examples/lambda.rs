use serde_yaml::Value as Yaml;
use yamlfun::Expr;
use yamlfun::Vm;

const ONE: &str = r#"
lambda: []
do: {$: 1}
"#;

const TWO: &str = r#"
lambda: []
do:
  lambda: []
  do: {$: 2}
"#;

const SUM: &str = r#"
lambda: [arg2, arg1]
do:
  +: [arg1, arg2]
"#;

fn main() {
    let vm = Vm::new();

    let one: Expr = serde_yaml::from_str(ONE.trim()).unwrap();
    let one = vm.eval(one.clone()).unwrap();
    let one = one.call([]).unwrap();
    println!("{:?}", &one);

    let two: Expr = serde_yaml::from_str(TWO.trim()).unwrap();
    let two = vm.eval(two.clone()).unwrap();
    let two = two.call([]).unwrap();
    let two = two.call([]).unwrap();
    println!("{:?}", two);

    let args: Expr = serde_yaml::from_str(SUM.trim()).unwrap();
    let args = vm.eval(args.clone()).unwrap();
    println!("{:?}", &args);

    let args = args.call([]).unwrap();
    println!("{:?}", &args);

    let args = args.call([Yaml::Number(10.into()).into()]).unwrap();
    println!("{:?}", &args);

    let args = args.call([Yaml::Number(30.into()).into()]).unwrap();
    println!("{:?}", &args);

    let args: Expr = serde_yaml::from_str(SUM.trim()).unwrap();
    let args = vm.eval(args.clone()).unwrap();
    let args = args
        .call([
            Yaml::Number(1.into()).into(),
            Yaml::Number(101.into()).into(),
        ])
        .unwrap();
    println!("{:?}", &args);
}
