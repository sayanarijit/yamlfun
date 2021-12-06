use yamlfun::{yaml, Expr, Vm, DefaultPlatform};

const YES: &str = "
:if:
  :==: [:: 2, :: 2]
:then: {:: yes}
:else: {:: no}
";

const NO: &str = r"
:if:
  :==: [:: 1, :: 2]
:then: {:: yes}
:else: {:: no}
";

const NESTED: &str = r"
:if:
  :if: {:: true}
  :then: {:: true}
  :else: {:: false}
:then:
  :if: {:: true}
  :then: {:: nested}
  :else: {:: not nested}
:else: {:: not nested at all}
";

fn main() {
    let vm = Vm::new(DefaultPlatform);

    let yes: Expr = yaml::from_str(YES.trim()).unwrap();
    println!("{}", vm.eval(yes).unwrap());

    let no: Expr = yaml::from_str(NO.trim()).unwrap();
    println!("{}", vm.eval(no).unwrap());

    let nested: Expr = yaml::from_str(NESTED.trim()).unwrap();
    println!("{}", vm.eval(nested).unwrap());
}
