pub fn verse(n: u32) -> String {
    format!(
        "{}, {}.\n{}, {}.\n",
        bottles(n, true, true),
        bottles(n, false, false),
        beer_flow(n),
        bottles(bootle_sub(n), false, true)
    )
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = vec![];
    for i in end..=start {
        song.push(verse(i))
    }
    song.reverse();
    song.join("\n")
}

fn bottles(n: u32, first_cap: bool, is_on_the_wall: bool) -> String {
    format!(
        "{} of beer{}",
        match n {
            0 => format!(
                "{}o more bottles",
                match first_cap {
                    true => "N",
                    false => "n",
                }
            ),
            1 => String::from("1 bottle"),
            _ => format!("{} bottles", n),
        },
        wall(is_on_the_wall)
    )
}

fn wall(input: bool) -> String {
    match input {
        true => String::from(" on the wall"),
        false => String::new(),
    }
}

fn beer_flow(n: u32) -> String {
    match n {
        0 => String::from("Go to the store and buy some more"),
        _ => format!(
            "Take {} down and pass it around",
            match n {
                1 => "it",
                _ => "one",
            }
        ),
    }
}

fn bootle_sub(n: u32) -> u32 {
    match n.checked_sub(1) {
        Some(count) => count,
        None => 99,
    }
}
