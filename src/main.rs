use ast::Term;
use run::run;

mod api;
mod ast;
mod run;

fn main() {
    let term = Term::App(
        Box::new(Term::Lib(ast::Lib::Predict)),
        Box::new(Term::Lit("123".to_string())),
    );
    let val = term.eval(&vec![]);
    let res = run(val);
    println!("result: {}", res);
}
