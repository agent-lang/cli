use crate::{api::predict, ast::Val};

pub async fn run(val: Val) -> String {
    match val {
        Val::Func(param, _, _) => format!("Func {:?}", param),
        Val::Var(id) => format!("Var {}", id),
        Val::Lit(lit) => lit,
        Val::App(func, arg) => match *func {
            Val::Var(name) => match name.as_str() {
                _ => predict(Box::pin(run(*arg)).await).await,
            },
            _ => panic!("invalid application"),
        },
    }
}
