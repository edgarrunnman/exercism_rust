use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut register = HashMap::from([('(', 0), ('[', 0), ('{', 0)]);
    string.chars().into_iter().fold(register, |c| {})
}
