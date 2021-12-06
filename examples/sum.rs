use yamlfun::{yaml, DefaultPlatform, Expr, Vm};

const ONE: &str = ":+: [:: 1]";
const THREE: &str = ":+: [:: 1, :: 2]";

fn main() {
    let vm = Vm::new(DefaultPlatform);

    let one: Expr = yaml::from_str(ONE.trim()).unwrap();
    let one = vm.eval(one).unwrap();
    println!("{}", one);

    let three: Expr = yaml::from_str(THREE.trim()).unwrap();
    let three = vm.eval(three).unwrap();
    println!("{}", three);
}
