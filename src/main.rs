use std::rc::Rc;

use ast::Term;
use run::run;

use crate::{
    ast::{Arg, Param, Type, Val},
    gen::{gen, Hole, Traverse},
};

mod api;
mod ast;
mod gen;
mod run;

fn main() {
    let hole_param = Param(
        "result".into(),
        Type::Exact("String".into()).into(),
        "Result of question".into(),
        Term::Lit("answer".into()).into(),
    );
    let env = Rc::new(vec![Arg("predict".into(), Val::Lib("predict".into()))]);
    let predict_param = Param(
        "prefix".into(),
        Type::Exact("String".into()).into(),
        "Prefix of predicted string".into(),
        Term::Lit("prefix".into()).into(),
    );
    let ctx = Rc::new(vec![Param(
        "predict".into(),
        Type::Func(predict_param.clone(), Type::Exact("String".into()).into()).into(),
        "Predict what follows `prefix`".into(),
        Term::Func(predict_param, Term::Lit("prediction".into()).into()).into(),
    )]);
    let root_pos = Traverse {
        lens: Rc::new(|term| term),
        ctx: ctx.clone(),
        env: env.clone(),
    };
    let generated = gen(
        "Predict the next words of \"114514\"".into(),
        Hole {
            pos: root_pos.clone(),
            param: hole_param,
        },
        root_pos,
    );
    let val = generated.eval(env.clone());
    let res = run(val, env.clone());
    println!("result: {}", res);
}
