use yamlfun::Expr;
use yamlfun::Vm;

const ONE: &str = "$: 1";
const FOO: &str = "$: foo";
const BAR: &str = "$: {foo: bar, 1: 2}";
const BAZ: &str = "$: [baz, 1, true, {a: b}]";

fn main() {
    let vm = Vm::new();
    let one: Expr = serde_yaml::from_str(ONE.trim()).unwrap();
    let one = vm.eval(one).unwrap();
    println!("{}", one);

    let foo: Expr = serde_yaml::from_str(FOO.trim()).unwrap();
    let foo = vm.eval(foo).unwrap();
    println!("{}", foo);

    let bar: Expr = serde_yaml::from_str(BAR.trim()).unwrap();
    let bar = vm.eval(bar).unwrap();
    println!("{}", bar);

    let baz: Expr = serde_yaml::from_str(BAZ.trim()).unwrap();
    let baz = vm.eval(baz).unwrap();
    println!("{}", baz);
}
