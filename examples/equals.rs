use yamlfun::{yaml, DefaultPlatform, Expr, Vm};

const YES: &str = ":==: [:: 1, :: 1]";
const NO: &str = ":==: [:: 1, :: 2]";
const FOO: &str = ":==: [:: foo, :: foo]";
const BAR: &str = ":==: [:: foo, :: var]";

fn main() {
    let vm = Vm::new(DefaultPlatform).unwrap();

    let yes: Expr = yaml::from_str(YES.trim()).unwrap();
    let yes = vm.eval(yes).unwrap();
    println!("{}", &yes);

    let no: Expr = yaml::from_str(NO.trim()).unwrap();
    let no = vm.eval(no).unwrap();
    println!("{}", &no);

    let foo: Expr = yaml::from_str(FOO.trim()).unwrap();
    let foo = vm.eval(foo).unwrap();
    println!("{}", &foo);

    let bar: Expr = yaml::from_str(BAR.trim()).unwrap();
    let bar = vm.eval(bar).unwrap();
    println!("{}", &bar);
}
