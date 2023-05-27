pub fn brackets_are_balanced(string: &str) -> bool {
    let mut register = vec![];
    for next in string.chars() {
        match next {
            ')' | ']' | '}' => {
                if register.pop() != matching_bracket(next) {
                    return false;
                }
            }
            '(' | '[' | '{' => register.push(next),
            _ => {}
        }
    }
    register.len() == 0
}

fn matching_bracket(bracket: char) -> Option<char> {
    match bracket {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        _ => None,
    }
}
