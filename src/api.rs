use std::fmt::Display;

pub fn predict(prefix: String) -> String {
    format!("predict({})", prefix)
}

pub fn ask(question: String) -> String {
    format!("predict({})", question)
}

pub fn choose<T: Display>(desc: String, opt: Vec<T>) -> String {
    format!(
        "choose({}, {})",
        desc,
        opt.iter()
            .map(|t| t.to_string())
            .fold("".to_string(), |a, b| a + ", " + &b)
    )
}
