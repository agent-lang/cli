use ast::Term;
use run::run;

use crate::ast::{Arg, Val};

mod api;
mod ast;
mod gen;
mod run;

fn main() {
    let term = Term::App(
        Box::new(Term::Var("predict".to_string())),
        Box::new(Term::Lit("123".to_string())),
    );
    let env = vec![Arg("predict".to_string(), Val::Lib("predict".to_string()))];
    let val = term.eval(&env);
    let res = run(val, &env);
    println!("result: {}", res);
}
