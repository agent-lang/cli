use std::fmt::Display;

pub trait Term<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn eval(&self) -> T;
}

impl<T> Display for dyn Term<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

pub struct Lit(String);
pub struct Predict(Box<dyn Term<String>>);
pub struct Ask(Box<dyn Term<String>>);
pub struct Code(Box<dyn Term<String>>, Box<dyn Term<String>>);
pub struct Hole;

impl Term<String> for Lit {
    fn eval(&self) -> String {
        let Lit(id) = self;
        id.to_string()
    }
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Lit(id) = self;
        write!(f, "{}", id)
    }
}

impl Term<String> for Predict {
    fn eval(&self) -> String {
        let Predict(prefix) = self;
        format!("predict({})", prefix.eval())
    }
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Predict(prefix) = self;
        write!(
            f,
            "/* The next predicted words of `prefix` */ predict(prefix = {})",
            prefix
        )
    }
}

impl Term<String> for Ask {
    fn eval(&self) -> String {
        let Ask(question) = self;
        format!("ask({})", question.eval())
    }
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Ask(question) = self;
        write!(
            f,
            "/* User's answer to `question` */ ask(question = {})",
            question
        )
    }
}

impl Term<String> for Code {
    fn eval(&self) -> String {
        let Code(language, description) = self;
        format!("code({}, {})", language.eval(), description.eval())
    }
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Code(language, description) = self;
        write!(
            f,
            "/* The code in `language` according to `description` */ code(language = {}, description = {})",
            language, description
        )
    }
}

impl Term<String> for Hole {
    fn eval(&self) -> String {
        format!("_")
    }
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<placeholder>")
    }
}
