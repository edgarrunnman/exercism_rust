/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits_opt: Vec<Option<u32>> = code
        .replace(" ", "")
        .chars()
        .into_iter()
        .rev()
        .enumerate()
        .map(|item| match item {
            (i, n) => {
                if i % 2 != 0 {
                    try_double(n.to_digit(10))
                } else {
                    n.to_digit(10)
                }
            }
        })
        .collect();
    if digits_opt.contains(&None::<u32>) || (digits_opt.len() == 1 && digits_opt[0] == Some(0)) {
        return false;
    }
    digits_opt.into_iter().sum::<Option<u32>>().unwrap() % 10 == 0
}

fn try_double(nunmber: Option<u32>) -> Option<u32> {
    match nunmber {
        Some(n) => {
            if n > 4 {
                Some(n * 2 - 9)
            } else {
                Some(n * 2)
            }
        }
        None => None,
    }
}
