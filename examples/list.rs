use yamlfun::Expr;
use yamlfun::Vm;

const LIST: &str = r#"
list:
  - {$: a}
  - {$: 1}
  - {$: 1.1}
  - {$: -1}
  - {$: true}
  - list:
      - {$: nested}
"#;

fn main() {
    let vm = Vm::new();

    let rec: Expr = serde_yaml::from_str(LIST.trim()).unwrap();
    println!("{}", vm.eval(rec).unwrap());
}
