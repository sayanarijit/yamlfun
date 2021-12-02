use yamlfun::Expr;
use yamlfun::Vm;

const ZERO: &str = "+: []";
const ONE: &str = "+: [$: 1]";
const THREE: &str = "+: [$: 1, $: 2]";

fn main() {
    let vm = Vm::new();

    let zero: Expr = serde_yaml::from_str(ZERO.trim()).unwrap();
    let zero = vm.eval(zero).unwrap();
    println!("{}", zero);

    let one: Expr = serde_yaml::from_str(ONE.trim()).unwrap();
    let one = vm.eval(one).unwrap();
    println!("{}", one);

    let three: Expr = serde_yaml::from_str(THREE.trim()).unwrap();
    let three = vm.eval(three).unwrap();
    println!("{}", three);
}
