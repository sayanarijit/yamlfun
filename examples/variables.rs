use serde_yaml::Value as Yaml;
use yamlfun::Expr;
use yamlfun::Vm;

const A: &str = "a";
const B: &str = "b";

fn main() {
    let vm = Vm::new().with_env([
        ("a".into(), Yaml::String("foo".into()).into()),
        ("b".into(), Expr::Var("a".into())),
    ]);

    let a: Expr = serde_yaml::from_str(A.trim()).unwrap();
    println!("{:?}", vm.eval(a).unwrap());

    let b: Expr = serde_yaml::from_str(B.trim()).unwrap();
    println!("{:?}", vm.eval(b).unwrap());
}
