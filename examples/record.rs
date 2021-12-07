use yamlfun::{yaml, DefaultPlatform, Expr, Vm};

const REC: &str = r#"
:let:
  foo:
    :rec:
      a:
        :rec:
          b: {:: {1: bar, true: baz}}
          "10": {:: foo}
      e: {:: {y: z}}

  betterFoo:
    :update: foo
    :set:
      a: {:: bar}
      oldFoo: foo
    :unset: [e]

:in:
  :rec:
    one:
      :case: foo
      :of:
        :rec:
          :as: {foo1: {:: a.b.$1}}
          :do: foo1
    $1: foo.a.b.$1
    $true: foo.a.b.$true
    y: foo.e.y
    "10": foo.a.10
    f: 
      :lambda: [a, b]
      :do:
        :+: [a, b]
    betterFoo: betterFoo
    got:
      :|>:
        - foo
        - [Rec.get, {:: a}]
        - [Maybe.withDefault, {:: null}]
"#;

fn main() {
    let vm = Vm::new(DefaultPlatform).unwrap();

    let rec: Expr = yaml::from_str(REC.trim()).unwrap();
    let rec = vm.eval(rec).unwrap();
    println!("{}", &rec);
}
