use std::rc::Rc;

use crate::{
    api::{ask, predict},
    ast::{Env, Val},
};

/// Run a value so that library functions are evaluated
/// The result will be optionally stored in a variable, so `env`
/// will be used multiple times.
pub fn run(val: Val, env: Rc<Env>) -> Val {
    match val {
        Val::App(func, mut args) => match *func {
            Val::Lib(name) if name == "predict" && args.len() == 1 => {
                let prefix = run(args.pop().unwrap(), env).to_string();
                Val::Lit(predict(prefix))
            }
            Val::Lib(name) if name == "ask" && args.len() == 1 => {
                let question = run(args.pop().unwrap(), env).to_string();
                Val::Lit(ask(question))
            }
            _ => panic!("invalid application"),
        },
        otherwise => otherwise,
    }
}
