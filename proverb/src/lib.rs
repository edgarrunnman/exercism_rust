pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let mut lines = list
        .windows(2)
        .map(|w| format!("For want of a {} the {} was lost.", w[0], w[1]))
        .collect::<Vec<String>>();
    lines.push(format!("And all for the want of a {}.", list[0]));
    lines.join("\n")
}
