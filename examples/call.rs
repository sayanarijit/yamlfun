use yamlfun::Expr;
use yamlfun::Vm;

const SUM: &str = "
():
  - lambda: [a, b]
    do:
      +: [a, b]
  - $: 10
  - $: 20
";

fn main() {
    let vm = Vm::new();

    let sum: Expr = serde_yaml::from_str(SUM.trim()).unwrap();
    println!("{:?}", &sum);
    let sum: usize = vm.eval(sum).unwrap().parse().unwrap();
    println!("{:?}", sum);
}
