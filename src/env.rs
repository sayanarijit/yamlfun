use crate::expr::Expr;
use indexmap::IndexMap;

pub type Env = IndexMap<String, Expr>;
