use yamlfun::Expr;
use yamlfun::Var;
use yamlfun::Vm;

const A: &str = "$: a";
const B: &str = "$: b";

fn main() {
    let mut vm = Vm::new().with_env([
        ("a".into(), Expr::Str("foo".into())),
        ("b".into(), Expr::Var(Var::new("a".into()))),
    ]);

    let a: Expr = serde_yaml::from_str(A.trim()).unwrap();
    println!("{:?}", vm.eval(a).unwrap());

    let b: Expr = serde_yaml::from_str(B.trim()).unwrap();
    println!("{:?}", vm.eval(b).unwrap());
}
