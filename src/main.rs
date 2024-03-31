use ast::Term;
use run::run;

use crate::ast::{Arg, Val};

mod api;
mod ast;
mod gen;
mod run;

fn main() {}

async fn test() {
    let term = Term::App(
        Box::new(Term::Var("predict".to_string())),
        Box::new(Term::Lit("123".to_string())),
    );
    let val = term.eval(&vec![Arg(
        "predict".to_string(),
        Val::Var("predict".to_string()),
    )]);
    let res = run(val).await;
    println!("result: {}", res);
}
