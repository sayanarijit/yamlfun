use crate::expr::Expr;
use std::collections::HashMap;

pub type Env = HashMap<String, Expr>;
