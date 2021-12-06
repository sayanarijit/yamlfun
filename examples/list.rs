use yamlfun::{yaml, DefaultPlatform, Expr, Vm};

const LIST: &str = r#"
:list:
  - {:: a}
  - {:: 1}
  - {:: 1.1}
  - {:: -1}
  - {:: true}
  - :list:
      - {:: nested}
"#;

fn main() {
    let vm = Vm::new(DefaultPlatform);

    let rec: Expr = yaml::from_str(LIST.trim()).unwrap();
    println!("{}", vm.eval(rec).unwrap());
}
