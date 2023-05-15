use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut register = HashMap::from([('(', 0), ('[', 0), ('{', 0)]);
    for c in string.chars() {
        match c {
            '(' => register[&'('] += 1,
            ')' => register[&'('] -= 1,
            '{' => register[&'{'] += 1,
            '}' => register[&'}'] -= 1,
            '[' => register[&'['] += 1,
            ']' => register[&']'] -= 1,
        }
    }
    register[&'('] == 0 && register[&'['] == 0 && register[&'{'] == 0
}
