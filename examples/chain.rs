use yamlfun::{yaml, DefaultPlatform, Expr, Vm};

const TEN: &str = "
:let:
  (+):
    :lambda: [x, y]
    :do:
      :+: [x, y]
:in:
  :|>:
    - { :: 1 }
    - [(+), { :: 5 }]
    - [(+), { :: 4 }]
";

fn main() {
    let vm = Vm::new(DefaultPlatform);

    let ten: Expr = yaml::from_str(TEN.trim()).unwrap();
    println!("{}", vm.eval(ten).unwrap());
}
