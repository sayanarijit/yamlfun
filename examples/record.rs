use yamlfun::Expr;
use yamlfun::Vm;

const REC: &str = r#"
let:
  foo:
    rec:
      a:
        rec:
          b:
            rec:
              c: {$: 1}
          d: {$: foo}
      e: {$: {y: z}}
in:
  rec:
    c: {.: [foo, a, b, c]}
    y: foo.e.y
    d: foo.a.d
"#;

fn main() {
    let vm = Vm::new();

    let rec: Expr = serde_yaml::from_str(REC.trim()).unwrap();
    let rec = vm.eval(rec).unwrap();
    println!("{}", &rec);
}
