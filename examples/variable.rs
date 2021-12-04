use yamlfun::{yaml, DefaultPlatform, Expr, Value, Vm};

const A: &str = "a";
const B: &str = "b";

fn main() {
    let vm = Vm::new(DefaultPlatform).with_env([
        ("a".into(), Value::String("foo".into()).into()),
        ("b".into(), Expr::Variable("a".into())),
    ]);

    let a: Expr = yaml::from_str(A.trim()).unwrap();
    println!("{}", vm.eval(a).unwrap());

    let b: Expr = yaml::from_str(B.trim()).unwrap();
    println!("{}", vm.eval(b).unwrap());
}
