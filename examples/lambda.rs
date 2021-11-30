use yamlfun::Expr;
use yamlfun::Vm;

const IS_TRUE: &str = "
lambda: [ boolVal ]
do:
  if:
    $: boolVal
  then: yes
  else: no
";

fn main() {
    let mut vm = Vm::new();

    let is_true: Expr = serde_yaml::from_str(IS_TRUE.trim()).unwrap();
    let func = vm.eval(is_true).unwrap();

    let yes = func.clone().call([Expr::Bool(true)]).unwrap();
    println!("{:?}", &yes);
    println!("{:?}", vm.eval(yes).unwrap());

    let no = func.call([Expr::Bool(false)]).unwrap();
    println!("{:?}", &no);
    println!("{:?}", vm.eval(no).unwrap());
}
