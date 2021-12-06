use anyhow::Result;
use std::fs;
use std::io::{self, Read};
use yamlfun::{yaml, DefaultPlatform, Expr, Vm};

fn main() -> Result<()> {
    let vm = Vm::new(DefaultPlatform)?;

    let code = if let Some(file) = std::env::args().skip(1).next() {
        fs::read_to_string(file)?
    } else {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut code = String::new();
        stdin.read_to_string(&mut code)?;
        code
    };

    let expr: Expr = yaml::from_str(&code)?;
    let res = vm.eval(expr).expect("failed to run");
    println!("{}", res);
    Ok(())
}
