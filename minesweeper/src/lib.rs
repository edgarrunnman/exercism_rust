pub fn annotate(minefield: &[&str]) -> Vec<String> {
    (0..minefield.len())
        .map(|x| {
            (0..minefield[x].len())
                .map(|y| match minefield[x].as_bytes()[y] {
                    42 => '*',
                    _ => count_mines(minefield, x as i8, y as i8) as char,
                })
                .collect()
        })
        .collect()
}

fn count_mines(minefield: &[&str], index_x: i8, index_y: i8) -> u8 {
    (-1..2 as i8)
        .fold(None as Option<u8>, |acc, x| {
            (-1..2 as i8).fold(acc, |mines, y| {
                if is_mine(minefield, index_x + x, index_y + y) {
                    add_one(mines)
                } else {
                    mines
                }
            })
        })
        .unwrap_or(32)
}

fn add_one(mines: Option<u8>) -> Option<u8> {
    match mines {
        Some(n) => Some(n + 1),
        None => Some(49),
    }
}

fn is_mine(minefield: &[&str], index_x: i8, index_y: i8) -> bool {
    minefield
        .iter()
        .nth(index_x as usize)
        .and_then(|line| {
            line.bytes()
                .into_iter()
                .nth(index_y as usize)
                .and_then(|char| Some(char == 42))
        })
        .unwrap_or(false)
}
