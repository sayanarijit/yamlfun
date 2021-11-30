use std::collections::HashMap;
use yamlfun::Expr;

fn main() {
    let expr: Expr = serde_yaml::from_str(include_str!("../concept.yml")).unwrap();
    // println!("expr:    {:?}", &expr);

    let mut env = HashMap::new();

    let lambda = expr.eval(&mut env).unwrap();
    // println!("lambda:    {:?}", &lambda);

    let letin = lambda.call([Expr::Bool(true)]).unwrap();
    // println!("letin:   {:?}", &letin);

    let result = letin.eval(&mut env).unwrap();
    println!("result:   {:?}", result);
}
