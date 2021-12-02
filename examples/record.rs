use yamlfun::Expr;
use yamlfun::Vm;

const REC: &str = r#"
rec:
  a:
    rec:
      b:
        rec:
          c: {$: 1}
      d: {$: foo}
  e: {$: false}
"#;

fn main() {
    let vm = Vm::new();

    let rec: Expr = serde_yaml::from_str(REC.trim()).unwrap();
    println!("{}", vm.eval(rec).unwrap());
}
