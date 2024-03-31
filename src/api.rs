use std::fmt::Display;

pub async fn predict(text: String) -> String {
    format!("predict({})", text)
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
