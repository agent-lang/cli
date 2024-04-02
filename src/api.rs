/// Predict the next words of an prefix with LLM
pub fn predict(prefix: String) -> String {
    format!("predict({})", prefix)
}

/// Ask user a question
pub fn ask(question: String) -> String {
    format!("predict({})", question)
}

/// Choose from some options with LLM
pub fn choose(desc: String, trans: &Vec<String>) -> usize {
    todo!();
}

/// Fill the only blank in the template with LLM
pub fn fill(tmpl: String, blank: String) -> String {
    format!("fill({}, {})", tmpl, blank);
    todo!();
}
