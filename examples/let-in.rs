use yamlfun::Expr;
use yamlfun::Vm;

const LET_IN: &str = "
let:
  a: {$: foo}
  b: a
in:
  b
";

const OVERRIDE: &str = "
let:
  a: {$: foo}
  b:
    let:
      a: {$: bar}
    in:
      a
in:
  b
";

const RESTORE: &str = "
let:
  a: {$: foo}
  b:
    let:
      a: {$: bar}
    in:
      a
in:
  a
";

fn main() {
    let vm = Vm::new();

    let let_in: Expr = serde_yaml::from_str(LET_IN.trim()).unwrap();
    println!("{}", vm.eval(let_in).unwrap());

    let override_: Expr = serde_yaml::from_str(OVERRIDE.trim()).unwrap();
    println!("{}", vm.eval(override_).unwrap());

    let restore: Expr = serde_yaml::from_str(RESTORE.trim()).unwrap();
    println!("{}", vm.eval(restore).unwrap());
}
