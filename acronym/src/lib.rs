pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace(" - ", " ")
        .replace("-", " ")
        .replace("_", "")
        .replace("  ", " ")
        .split(" ")
        .fold("".to_string(), |acc, n| format!("{}{}", acc, next_chars(n)))
        .to_uppercase()
}
fn next_chars(str: &str) -> String {
    let chars = str
        .chars()
        .into_iter()
        .filter(|c| c.is_uppercase())
        .collect::<Vec<char>>();
    if chars.len() == 0 || chars.len() == str.len() {
        str.chars()
            .into_iter()
            .collect::<Vec<char>>()
            .first()
            .unwrap()
            .to_string()
    } else {
        chars.into_iter().collect::<String>()
    }
}
