use crate::{
    api::choose,
    ast::{Context, Term, Type},
};

pub async fn gen(desc: String, term: Term, ty: Type, ctx: Context) -> Term {
    let options: Vec<Term> = vec![];
    let choice = choose(
        format!("Which one satisfies most: \n```\n{}\n```", desc),
        options,
    )
    .await;
    Term::Lit("".to_string())
}
