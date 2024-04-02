use std::fmt::Display;

pub fn predict(prefix: String) -> String {
    format!("predict({})", prefix)
}

pub fn ask(question: String) -> String {
    format!("predict({})", question)
}

pub fn choose<I>(desc: String, opt: I) -> <I as Iterator>::Item
where
    I: Iterator,
    <I as Iterator>::Item: Display,
{
    format!(
        "choose({}, {})",
        desc,
        opt.map(|t| t.to_string())
            .fold("".to_string(), |a, b| a + ", " + &b)
    );
    todo!();
}
