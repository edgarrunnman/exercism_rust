pub fn raindrops(n: u32) -> String {
    let text = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .into_iter()
        .fold(String::new(), |acc, f| match n % f.0 {
            0 => format!("{}{}", acc, f.1),
            _ => acc,
        });
    match text.is_empty() {
        true => n.to_string(),
        false => text,
    }
}
