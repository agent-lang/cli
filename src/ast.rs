use std::fmt::Display;

/// Param = name: Type /* Description */
#[derive(Debug, Clone)]
pub struct Param(pub String, pub String, pub Box<Type>);

/// Param can be displayed
impl Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} /* {} */", self.0, self.2, self.1)
    }
}

/// Type describes the property an operation satisfies
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum Term {
    /// Function: param => returnTerm
    Func(Param, Box<Term>),

    /// Variable
    Var(String),

    /// Literal
    Lit(String),

    /// Application: func arg
    App(Box<Term>, Box<Term>),
}

/// Term can be displayed
impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Func(param, term) => write!(f, "({}) => ({})", param, term),
            Term::Var(id) => write!(f, "{}", id),
            Term::Lit(lit) => write!(f, "\"{}\"", lit),
            Term::App(func, arg) => write!(f, "({})({})", func, arg),
        }
    }
}

/// Term can be evaluated to value
impl Term {
    pub fn eval(self, env: &Env) -> Val {
        match self {
            Term::Func(param, next) => Val::Func(param, next, env.clone()),
            Term::Var(id) => match env.iter().find(|v| v.0 == id) {
                Some(val) => val.1.clone(),
                None => panic!("variable not found"),
            },
            Term::Lit(lit) => Val::Lit(lit),
            Term::App(func, arg) => {
                let eval = func.eval(env);
                eval.apply(arg.eval(env))
            }
        }
    }
}

/// Value is the semantics of a term
#[derive(Debug, Clone)]
pub enum Val {
    /// Function encloses a term with its environment
    Func(Param, Box<Term>, Env),

    /// Variable
    Var(String),

    /// Library function,
    Lib(String),

    /// Literal
    Lit(String),

    /// Application: func args
    App(Box<Val>, Vec<Val>),
}

/// Value can be applied with argument
impl Val {
    fn apply(self, arg: Val) -> Val {
        match self {
            Val::Func(param, next, mut env) => {
                // Push argument to environment
                env.push(Arg(param.0, arg));

                // Evaluate with complete environment
                next.eval(&env)
            }
            Val::App(func, mut args) => {
                // Push argument to list
                args.push(arg);

                // Return self
                Val::App(func, args)
            }
            val => Val::App(Box::new(val), vec![arg]),
        }
    }
}

/// Value can be displayed
impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Func(param, body, _env) => write!(f, "({}) => ({})", param, body),
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
        }
    }
}

/// Context stores the mapping from name to type
pub type Context = Vec<Param>;

/// Env stores the mapping from name to value
pub type Env = Vec<Arg>;

/// Argument: name = value
#[derive(Clone, Debug)]
pub struct Arg(pub String, pub Val);
