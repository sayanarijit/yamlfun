use yamlfun::Expr;
use yamlfun::Vm;

const ADD: &str = "+: [1, 2]";
const ADD_ONE: &str = "+: [1]";

fn main() {
    let mut vm = Vm::new();

    let add: Expr = serde_yaml::from_str(ADD.trim()).unwrap();
    println!("{:?}", &add);
    println!("{:?}", vm.eval(add).unwrap());

    let add_one: Expr = serde_yaml::from_str(ADD_ONE.trim()).unwrap();
    println!("{:?}", &add_one);
    println!("{:?}", vm.call(add_one, [Expr::Number(4)]).unwrap());
}
