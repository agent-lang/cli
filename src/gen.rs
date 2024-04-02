use std::rc::Rc;

use crate::{
    api::{choose, fill},
    ast::{Arg, Ctx, Env, Param, Term, Type, Val},
};

/// Position contains all relevant info about a position in an AST
#[derive(Clone)]
pub struct Position<'a> {
    pub lens: Rc<dyn Fn(Term) -> Term + 'a>,
    pub ctx: Rc<Ctx>,
    pub env: Rc<Env>,
}

/// Position's lens and context can be extended
impl<'a> Position<'a> {
    fn map(self, f: impl Fn(Term) -> Term + 'a) -> Self {
        Self {
            lens: Rc::new(move |term| (self.lens)(f(term))),
            ctx: self.ctx,
            env: self.env,
        }
    }

    fn with(self, param: &Param) -> Self {
        // Generate new context and environment
        // Clone is needed because we can create different branches
        // Performance impact is O(n^2), can be optimized
        let mut new_ctx = (*self.ctx).clone();
        new_ctx.push(param.clone());
        let mut new_env = (*self.env).clone();
        new_env.push(Arg(param.0.clone(), Val::Var(param.0.clone()).into()));

        // Apply new context and environment
        Self {
            lens: self.lens,
            ctx: new_ctx.into(),
            env: new_env.into(),
        }
    }
}

/// Hole contains info of an AST position and info of the hole term
#[derive(Clone)]
pub struct Hole<'a> {
    pub pos: Position<'a>,
    pub param: Param,
}

/// Convert `param` to a hint, if it can be constructed as a instance of `ty`
pub fn as_hint(param: &Param, ty: &Type) -> Option<Term> {
    let Param(name, pty, desc) = param;
    // Clone is needed because pty can form the term returned
    // Performance is O(n)
    match *pty.clone() {
        // A function with correct return type can be hinted as `f <a> ... <z>`
        Type::Func(param, ret) if *ret == *ty => {
            // Generate hint from return type first
            as_hint(&Param(name.clone(), ret, desc.clone()), ty).map(|term| {
                // Apply inner hint with hole argument
                Term::App(term.into(), Term::Hole(param).into())
            })
        }
        // A variable with correct type is a trivial hint
        pty if pty == *ty => Some(Term::Var(name.clone())),
        _ => None,
    }
}

/// Generate more hints from type
pub fn more_hints(hints: &mut Vec<Term>, ty: &Type) {
    match ty {
        Type::Func(param, ty) => {
            // Clone is needed because pty can form the term added to `hints`
            // Performance is O(n)
            let hole = Term::Hole(Param(
                "func_body".into(),
                ty.clone(),
                "Function Body".into(),
            ));
            hints.push(Term::Func(param.clone(), hole.into()).into())
        }
        Type::Exact(ty) if ty == "String" => hints.push(Term::Lit("{SOME_STRING}".into())),
        _ => (),
    }
}

/// The first hole of a given term
pub fn first_hole<'a>(term: &'a Term, pos: Position<'a>) -> Option<Hole<'a>> {
    match term {
        Term::Func(param, body) => {
            // Get body pos
            let body_pos = pos
                .map(|term| Term::Func(param.clone(), term.into()))
                .with(param);

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
            first_hole(func, func_pos).or_else(|| first_hole(arg, arg_pos))
        }
        Term::Let(param, body, next) => {
            // Get body and next pos
            let cloned_pos = pos.clone();
            let body_pos = pos
                .map(|term| Term::Let(param.clone(), term.into(), next.clone()))
                .with(param);
            let next_pos = cloned_pos
                .map(|term| Term::Let(param.clone(), body.clone(), term.into()))
                .with(param);

            // Look into body first, and then next
            first_hole(body, body_pos).or_else(|| first_hole(next, next_pos))
        }
        Term::Hole(param) => Some(Hole {
            pos,
            param: param.clone(),
        }),
    }
}

/// Generate a term with all info about a hole
/// Automatically looks for the next hole
pub fn gen<'a>(desc: String, hole: Hole<'a>, root: Position<'a>) -> Term {
    // Generate options from context
    let ctx = hole.pos.ctx.iter();
    let ty = hole.param.1;
    let mut options: Vec<_> = ctx.filter_map(|param| as_hint(param, &ty)).collect();

    // Generate more options from type
    more_hints(&mut options, &ty);

    // Choose from all valid hints
    let mut preview: Vec<_> = options
        .iter()
        // Clone is needed because the "preview term" must be built elsewhere
        // Performance impact is O(n * option_count)
        .map(|opt| (hole.pos.lens)(opt.clone()))
        .collect();
    let translation: Vec<_> = preview.iter().map(|prev| prev.to_string()).collect();
    let choice_ix = choose(
        format!("Which one satisfies most: \n```\n{}\n```", desc),
        &translation,
    );

    // Generate final choice from index
    // Undefault string literal if needed
    let choice = if let Term::Lit(ref lit) = options[choice_ix] {
        (hole.pos.lens)(Term::Lit(fill(translation[choice_ix].clone(), lit.into())))
    } else {
        preview.swap_remove(choice_ix)
    };

    // Generate the next hole, if any
    // Clone is needed because a `Hole` can take the ownership of mutated `root`
    // Performance impact is O(n)
    if let Some(next_hole) = first_hole(&choice, root.clone()) {
        return gen(desc, next_hole, root);
    }

    // Otherwise directly return the choice
    // Did not use the `else` branch, so that `first_hole(...)` is destructed
    choice
}
