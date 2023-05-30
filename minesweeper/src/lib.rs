pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let foo = minefield
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|symbol| symbol as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    println!("{:?}", foo);
    Vec::new()
}
