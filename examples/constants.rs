use yamlfun::Expr;
use yamlfun::Vm;

const ONE: &str = "$: 1";
const FOO: &str = "$: foo";

fn main() {
    let vm = Vm::new();
    let one: Expr = serde_yaml::from_str(ONE.trim()).unwrap();
    let one: i64 = vm.eval(one.clone()).unwrap().parse().unwrap();
    println!("{}", one);

    let foo: Expr = serde_yaml::from_str(FOO.trim()).unwrap();
    let foo: String = vm.eval(foo.clone()).unwrap().parse().unwrap();
    println!("{}", foo);
}
