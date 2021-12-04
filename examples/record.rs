use yamlfun::{Expr, Vm, DefaultPlatform, yaml};

const REC: &str = r#"
let:
  foo:
    rec:
      a:
        rec:
          b: {:: {1: bar, true: baz}}
          "10": {:: foo}
      e: {:: {y: z}}
in:
  rec:
    one:
      .: [foo, {:: a}, {:: b}, {:: 1}]
    (1): foo.a.b.(1)
    (true): foo.a.b.(true)
    y: foo.e.y
    "10": foo.a.10
    f: 
      lambda: [a, b]
      do:
        +: [a, b]
"#;

fn main() {
    let vm = Vm::new(DefaultPlatform);

    let rec: Expr = yaml::from_str(REC.trim()).unwrap();
    let rec = vm.eval(rec).unwrap();
    println!("{}", &rec);
}
