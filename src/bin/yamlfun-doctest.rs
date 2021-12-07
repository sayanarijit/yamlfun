use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json as json;
use std::fs;
use std::io::{self, Read};
use yamlfun::{yaml, DefaultPlatform, Expr, Value, Vm};

#[derive(Debug, Deserialize, Serialize)]
struct Test {
    #[serde(rename = "Test", alias = "Example")]
    name: String,

    #[serde(rename = "Run")]
    run: Expr,

    #[serde(rename = "Expect", alias = "Result")]
    result: json::Value,
}

fn main() -> Result<()> {
    let mut vm = Vm::new(DefaultPlatform)?;

    let code = if let Some(file) = std::env::args().skip(1).next() {
        fs::read_to_string(file)?
    } else {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut code = String::new();
        stdin.read_to_string(&mut code)?;
        code
    };

    let env: Value = vm.eval(yaml::from_str(&code)?)?;
    if let Value::Record(rec) = env {
        vm = vm.with_env(rec.iter().map(|(k, v)| (k.to_string(), v.clone().into())));
    };

    let tests = code
        .lines()
        .map(|l| l.trim_start())
        .filter_map(|l| l.strip_prefix("#: "))
        .map(String::from)
        .collect::<Vec<String>>()
        .join("\n");

    let tests: Vec<Test> = yaml::from_str(&tests)?;

    for (i, test) in tests.into_iter().enumerate() {
        println!();
        println!("Test {}: {}", i + 1, &test.name);
        println!();
        println!("  Running:   {}", &test.run);
        println!("  Expecting: {}", &test.result);

        let res = vm.eval(test.run)?;
        let res = json::to_value(res)?;

        let expect = json::to_value(test.result)?;

        println!("  Got:       {}", &res);
        let status = if res == expect { "success" } else { "!!!FAILED!!!" };
        println!("  Status:    {}", status);
        println!();
    }
    Ok(())
}
