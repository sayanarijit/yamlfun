use yamlfun::{Expr, Vm};

const SUM: &str = "
- lambda: [a, b]
  do:
    +: [a, b]
- :: 10
- :: 20
";

fn main() {
    let vm = Vm::default();

    let sum: Expr = serde_yaml::from_str(SUM.trim()).unwrap();
    let sum = vm.eval(sum).unwrap();
    println!("{}", sum);
}
