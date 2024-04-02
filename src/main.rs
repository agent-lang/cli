use std::rc::Rc;

use run::run;

use crate::{
    ast::{Arg, Param, Type, Val},
    gen::{gen, Hole, Position},
};

mod api;
mod ast;
mod gen;
mod run;

fn main() {
    // Generate environment and context
    let hole_param = Param(
        "result".into(),
        Type::Exact("String".into()).into(),
        "Result of question".into(),
    );
    let predict_param = Param(
        "prefix".into(),
        Type::Exact("String".into()).into(),
        "Prefix of predicted string".into(),
    );
    let env = Rc::new(vec![Arg(
        "predict".into(),
        Val::Lib("predict".into()).into(),
    )]);
    let ctx = Rc::new(vec![Param(
        "predict".into(),
        Type::Func(predict_param.clone(), Type::Exact("String".into()).into()).into(),
        "Predict what follows `prefix`".into(),
    )]);

    // Root position of the AST
    let root_pos = Position {
        lens: Rc::new(|term| term),
        ctx: ctx.clone(),
        env: env.clone(),
    };

    // Generate AST with hole info
    let generated = gen(
        "Predict the next words of \"114514\"".into(),
        Hole {
            pos: root_pos.clone(),
            param: hole_param,
        },
        root_pos,
    );

    // Evaluate generated AST
    let val = generated.eval(env.clone());

    // Run generated AST
    let res = run(val, env.clone());

    // Print result
    println!("result: {}", res);
}
