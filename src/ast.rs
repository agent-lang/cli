/// Param = name -Description-: Type
#[derive(Debug, Clone)]
pub struct Param(String, String, Box<Type>);

/// Type describes the property an operation satisfies
#[derive(Debug, Clone)]
pub enum Type {
    /// Function: param -> ReturnType
    Func(Param, Box<Type>),

    /// Exact is type literal
    Exact(String),
}

/// Library term
#[derive(Debug, Clone)]
pub enum Lib {
    /// LLM Eval
    Predict,
}

/// Term describes what an operation is
#[derive(Debug, Clone)]
pub enum Term {
    /// Function: param => returnTerm
    Func(Param, Box<Term>),

    /// Library term'
    Lib(Lib),

    /// Variable
    Var(String),

    /// Literal
    Lit(String),

    /// Application: func arg
    App(Box<Term>, Box<Term>),
}

/// Term can be evaluated to value
impl Term {
    pub fn eval(self, env: &Env) -> Val {
        match self {
            Term::Func(param, next) => Val::Func(param, next, env.clone()),
            Term::Lib(lib) => Val::Lib(lib),
            Term::Var(id) => match env.iter().find(|v| v.0 == id) {
                Some(val) => val.1.clone(),
                None => todo!("variable not found"),
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

    /// Library term
    Lib(Lib),

    /// Variable
    Var(String),

    /// Literal
    Lit(String),

    /// Application: func arg
    App(Box<Val>, Box<Val>),
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
            val => Val::App(Box::new(val), Box::new(arg)),
        }
    }
}

/// Context stores the mapping from name to type
pub type Context = Vec<Param>;

/// Env stores the mapping from name to value
pub type Env = Vec<Arg>;

/// Argument: name = value
#[derive(Clone, Debug)]
pub struct Arg(String, Val);
