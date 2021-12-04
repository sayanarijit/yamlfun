use yamlfun::{yaml, DefaultPlatform, Expr, Vm};

const WITH: &str = r#"
let:
  args1:
    rec:
      first: {:: 10}
      second: {:: 20}
  args2:
    ::
      third: 30
in:
  with: [args1, args2]
  do:
    +: [first, second, third]
"#;

fn main() {
    let vm = Vm::new(DefaultPlatform);

    let with: Expr = yaml::from_str(WITH.trim()).unwrap();
    let with = vm.eval(with).unwrap();
    println!("{}", &with);
}
