use crate::{
    api::{ask, predict},
    ast::Val,
};

pub async fn run(val: Val) -> String {
    match val {
        Val::Func(param, _, _) => format!("Func {:?}", param),
        Val::Var(id) => format!("Var {}", id),
        Val::Lit(lit) => lit,
        Val::App(func, args) => match *func {
            Val::Var(name) if name == "predict" && args.len() == 1 => {
                predict(Box::pin(run(args[0].clone())).await).await
            }
            Val::Var(name) if name == "ask" && args.len() == 1 => {
                ask(Box::pin(run(args[0].clone())).await).await
            }
            _ => panic!("invalid application"),
        },
    }
}
