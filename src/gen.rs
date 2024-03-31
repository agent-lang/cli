use crate::{
    api::choose,
    ast::{Term, Type, Context},
};

pub fn 

pub async fn gen(desc: String, term: Term, ty: Type, ctx: Context) -> Term {
    let options: Vec<Term> = vec![];
    let choice = choose(
        format!("Which one satisfies most: \n```\n{}\n```", desc),
        options,
    )
    .await;
}
