use std::fmt::Display;

pub async fn predict(prefix: String) -> String {
    format!("predict({})", prefix)
}

pub async fn ask(question: String) -> String {
    format!("predict({})", question)
}

pub async fn choose<T: Display>(desc: String, opt: Vec<T>) -> String {
    format!(
        "choose({}, {})",
        desc,
        opt.iter()
            .map(|t| t.to_string())
            .fold("".to_string(), |a, b| a + ", " + &b)
    )
}
