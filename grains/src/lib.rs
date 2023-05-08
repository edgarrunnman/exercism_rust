pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    (1..s).into_iter().fold(1, |acc, _| acc * 2)
}

pub fn total() -> u64 {
    (1..64)
        .into_iter()
        .fold((1, 1), |acc, _| (acc.0 * 2, acc.1 + acc.0 * 2))
        .1
}
