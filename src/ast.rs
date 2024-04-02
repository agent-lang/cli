use std::{fmt::Display, rc::Rc};

/// Param = name: Type /* Description; Example: example */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param(pub String, pub Box<Type>, pub String, pub Box<Term>);

/// Param can be displayed
impl Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} /* {}; Example: {} */",
            self.0, self.1, self.2, self.3
        )
    }
}

/// Type describes the property an operation satisfies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Function: param -> ReturnType
    Func(Param, Box<Type>),

    /// Exact is type literal
    Exact(String),
}

/// Type can be displayed
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Func(param, ty) => write!(f, "({}) -> ({})", param, ty),
            Type::Exact(id) => write!(f, "{}", id),
        }
    }
}

/// Term describes what an operation is
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    /// Function: param => returnTerm
    Func(Param, Box<Term>),

    /// Variable
    Var(String),

    /// Literal
    Lit(String),

    /// Application: func arg
    App(Box<Term>, Box<Term>),

    /// Let: let (param) = term; next
    Let(Param, Box<Term>, Box<Term>),

    /// Hole: _
    Hole(Param),
}

/// Term can be displayed
impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Func(param, term) => write!(f, "({}) => ({})", param, term),
            Term::Var(id) => write!(f, "{}", id),
            Term::Lit(lit) => write!(f, "\"{}\"", lit),
            Term::App(func, arg) => write!(f, "({})({})", func, arg),
            Term::Let(param, term, next) => write!(f, "let ({}) = {};\n{}", param, term, next),
            Term::Hole(param) => write!(f, "<{}>", param),
        }
    }
}

/// Term can be evaluated to value
impl Term {
    pub fn eval(&self, env: Rc<Env>) -> Val {
        match self {
            Term::Func(param, next) => Val::Func(param.clone(), next.clone(), env.clone()),
            Term::Var(id) => match env.iter().find(|v| v.0 == *id) {
                Some(Arg(_, val)) => val.clone().unrec(id.clone()),
                None => panic!("variable not found"),
            },
            Term::Lit(lit) => Val::Lit(lit.clone()),
            Term::App(func, arg) => {
                let val = func.eval(env.clone());
                val.apply(arg.eval(env.clone()))
            }
            Term::Let(param, term, next) => {
                // Append recursive variable to a new environment
                let mut new_env = (*env).clone();
                new_env.push(Arg(param.0.clone(), Val::Rec(term.clone(), env.clone())));

                // Directly evaluate next term
                next.eval(new_env.into())
            }
            Term::Hole(param) => Val::Hole(param.clone()),
        }
    }
}

/// Value is the semantics of a term
#[derive(Debug, Clone)]
pub enum Val {
    /// Function encloses a term with its environment
    Func(Param, Box<Term>, Rc<Env>),

    /// Rec term is evaluated when needed
    /// The environment does NOT contain itself
    Rec(Box<Term>, Rc<Env>),

    /// Variable
    Var(String),

    /// Library function,
    Lib(String),

    /// Literal
    Lit(String),

    /// Application: func args
    App(Box<Val>, Vec<Val>),

    /// Hole
    Hole(Param),
}

/// Value can be applied with argument
impl Val {
    pub fn apply(self, arg: Val) -> Val {
        match self {
            Val::Func(param, next, env) => {
                // Push argument to a new environment
                let mut new_env = (*env).clone();
                new_env.push(Arg(param.0, arg));

                // Evaluate with complete environment
                next.eval(new_env.into())
            }
            Val::App(func, mut args) => {
                // Push argument to list
                args.push(arg);

                // Return self
                Val::App(func, args)
            }
            val => Val::App(val.into(), vec![arg]),
        }
    }

    pub fn unrec(self, id: String) -> Val {
        match self {
            Val::Rec(term, env) => {
                // Append recursive variable to a new environment
                let mut new_env = (*env).clone();
                new_env.push(Arg(id, Val::Rec(term.clone(), env.clone())));

                // Evaluate with new environment (no positive check)
                term.eval(new_env.into())
            }
            other => other,
        }
    }
}

/// Value can be displayed
impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Func(param, body, _env) => write!(f, "({}) => ({})", param, body),
            Val::Rec(term, _env) => write!(f, "{}", term),
            Val::Var(id) => write!(f, "{}", id),
            Val::Lib(id) => write!(f, "{}", id),
            Val::Lit(lit) => write!(f, "\"{}\"", lit),
            Val::App(func, arg) => {
                let arg_str = arg
                    .iter()
                    .map(|x| format!("({})", x))
                    .collect::<Vec<_>>()
                    .join("");
                write!(f, "({}){}", func, arg_str)
            }
            Val::Hole(param) => write!(f, "<{}>", param),
        }
    }
}

/// Context stores the mapping from name to type
pub type Ctx = Vec<Param>;

/// Env stores the mapping from name to value
pub type Env = Vec<Arg>;

/// Argument: name = value
#[derive(Clone, Debug)]
pub struct Arg(pub String, pub Val);
