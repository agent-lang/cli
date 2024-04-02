use std::rc::Rc;

use crate::{
    api::choose,
    ast::{Arg, Ctx, Env, Param, Term, Type, Val},
};

#[derive(Clone)]
pub struct Traverse<'a> {
    pub lens: Rc<dyn Fn(Term) -> Term + 'a>,
    pub ctx: Rc<Ctx>,
    pub env: Rc<Env>,
}

impl<'a> Traverse<'a> {
    fn map(self, f: impl Fn(Term) -> Term + 'a) -> Self {
        Self {
            lens: Rc::new(move |term| (self.lens)(f(term))),
            ctx: self.ctx,
            env: self.env,
        }
    }

    fn with(self, param: &Param, opt_val: Option<Val>) -> Self {
        // Generate new context and environment
        let mut new_ctx = (*self.ctx).clone();
        new_ctx.push(param.clone());
        let val = match opt_val {
            Some(v) => v,
            None => param.3.eval(self.env.clone()),
        };
        let mut new_env = (*self.env).clone();
        new_env.push(Arg(param.0.clone(), val));

        // Apply new context and environment
        Self {
            lens: self.lens,
            ctx: new_ctx.into(),
            env: new_env.into(),
        }
    }
}

#[derive(Clone)]
pub struct Hole<'a> {
    pub pos: Traverse<'a>,
    pub param: Param,
}

/// Convert `param` to a hint, if it can be constructed as a instance of `ty`
pub fn as_hint(param: &Param, ty: &Type) -> Option<Term> {
    let Param(name, pty, desc, eg) = param;
    match *pty.clone() {
        // A function with correct return type can be hinted as `f <a> ... <z>`
        Type::Func(param, ret) if *ret == *ty => {
            // Generate hint from return type first
            as_hint(&Param(name.clone(), ret, desc.clone(), eg.clone()), ty).map(|term| {
                // Apply inner hint with hole argument
                Term::App(term.into(), Term::Hole(param).into())
            })
        }
        // A variable with correct type is a trivial hint
        pty if pty == *ty => Some(Term::Var(name.clone())),
        _ => None,
    }
}

/// The first hole of a given term
pub fn first_hole<'a>(term: &'a Term, pos: Traverse<'a>) -> Option<Hole<'a>> {
    match term {
        Term::Func(param, body) => {
            // Get body pos
            let body_pos = pos
                .map(|term| Term::Func(param.clone(), term.into()))
                .with(param, None);

            // Look into body
            first_hole(body, body_pos)
        }
        Term::Var(_) => None,
        Term::Lit(_) => None,
        Term::App(func, arg) => {
            // Get func and arg pos
            let cloned_pos = pos.clone();
            let func_pos = pos.map(|term| Term::App(term.into(), arg.clone()));
            let arg_pos = cloned_pos.map(|term| Term::App(func.clone(), term.into()));

            // Look into function first, and then argument
            first_hole(func, func_pos).or(first_hole(arg, arg_pos))
        }
        Term::Let(param, body, next) => {
            // Get body and next pos
            let cloned_pos = pos.clone();
            let body_pos = pos
                .map(|term| Term::Let(param.clone(), term.into(), next.clone()))
                .with(param, None);
            let next_pos = cloned_pos
                .map(|term| Term::Let(param.clone(), body.clone(), term.into()))
                .with(param, None);

            // Look into body first, and then next
            first_hole(body, body_pos).or(first_hole(next, next_pos))
        }
        Term::Hole(param) => Some(Hole {
            pos,
            param: param.clone(),
        }),
    }
}

/// Generate a term with all info about a hole
/// Automatically looks for the next hole
pub fn gen<'a>(desc: String, hole: Hole<'a>, root: Traverse<'a>) -> Term {
    // Choose from all valid hints
    let ctx = hole.pos.ctx.iter();
    let options = ctx.filter_map(|param| as_hint(param, &hole.param.1));
    let choice = choose(
        format!("Which one satisfies most: \n```\n{}\n```", desc),
        options.map(|opt| (hole.pos.lens)(opt)),
    );

    // Generate the next whole, if any
    if let Some(next_hole) = first_hole(&choice, root.clone()) {
        return gen(desc, next_hole, root);
    }

    // Did not use the `else` branch, so that `first_hole(...)` is destructed
    choice
}
