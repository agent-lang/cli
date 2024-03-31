use crate::{
    api::predict,
    ast::{Lib, Val},
};

pub fn run(val: Val) -> String {
    match val {
        Val::Func(param, _, _) => format!("Func {:?}", param),
        Val::Lib(lib) => format!("Lib {:?}", lib),
        Val::Var(id) => format!("Var {}", id),
        Val::Lit(lit) => lit,
        Val::App(func, arg) => match *func {
            Val::Lib(Lib::Predict) => predict(run(*arg)),
            _ => format!("Apply {}", run(*arg)),
        },
    }
}
