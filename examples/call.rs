use yamlfun::{DefaultPlatform, Expr, Vm, yaml};

const SUM: &str = "
- :lambda: [a, b]
  :do:
    :+: [a, b]
- :: 10
- :: 20
";

fn main() {
    let vm = Vm::new(DefaultPlatform).unwrap();

    let sum: Expr = yaml::from_str(SUM.trim()).unwrap();
    let sum = vm.eval(sum).unwrap();
    println!("{}", sum);
}
