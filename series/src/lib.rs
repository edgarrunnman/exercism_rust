pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec![String::new(); digits.len() + 1],
        n => digits
            .chars()
            .collect::<Vec<char>>()
            .windows(n)
            .map(|chars| chars.into_iter().collect())
            .collect(),
    }
}
