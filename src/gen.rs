use std::rc::Rc;

use crate::{
    api::choose,
    ast::{Ctx, Env, Param, Term, Type},
};

#[derive(Clone)]
pub struct Traverse<'a> {
    lens: Rc<dyn Fn(Term) -> Term + 'a>,
    ctx: Rc<Ctx>,
    env: Rc<Env>,
}

impl<'a> Traverse<'a> {
    fn map(&'a self, f: Box<dyn Fn(Term) -> Term>) -> Self {
        Self {
            lens: Rc::new(move |term| f((self.lens)(term))),
            ctx: self.ctx.clone(),
            env: self.env.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Hole<'a> {
    pos: Traverse<'a>,
    param: Param,
}

pub fn as_hint(param: &Param, ty: &Type) -> Option<Term> {
    let Param(name, pty, desc, eg) = param;
    match *pty.clone() {
        // A function with correct return type can be hinted as `f <a> ... <z>`
        Type::Func(param, ret) if *ret == *ty => {
            // Generate hint from return type first
            as_hint(&Param(name.clone(), ret, desc.clone(), eg.clone()), ty).map(|term| {
                // Apply inner hint with hole argument
                Term::App(Box::new(term), Box::new(Term::Hole(param)))
            })
        }
        // A variable with correct type is a trivial hint
        pty if pty == *ty => Some(Term::Var(name.clone())),
        _ => None,
    }
}

/// Iterate all holes of a given term
pub fn holes<'a>(term: &'a Term, pos: Traverse<'a>) -> Vec<Hole<'a>> {
    match term {
        Term::Func(param, body) => holes(
            body,
            Traverse {
                lens: Rc::new(move |t| (pos.lens)(Term::Func(param.clone(), Box::new(t)))),
                ctx: pos.ctx.clone(),
                env: pos.env.clone(),
            },
        ),
        Term::Var(_) => todo!(),
        Term::Lit(_) => todo!(),
        Term::App(_, _) => todo!(),
        Term::Let(_, _, _) => todo!(),
        Term::Hole(param) => vec![Hole {
            pos: pos.clone(),
            param: param.clone(),
        }],
    }
}

/// Generate a term at `lens` with given info
/// `lens` converts inner term to complete term
pub async fn gen<'a>(desc: String, hole: Hole<'a>) -> Term {
    let ctx = hole.pos.ctx.iter();
    let options = ctx.filter_map(|param| as_hint(param, &hole.param.1));
    let choice = choose(
        format!("Which one satisfies most: \n```\n{}\n```", desc),
        options.map(|opt| (hole.pos.lens)(opt)),
    );
    todo!()
}
