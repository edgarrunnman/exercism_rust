pub fn is_leap_year(year: u64) -> bool {
    let divisible_by = (year % 4 == 0, year % 100 == 0, year % 400 == 0);
    match divisible_by {
        (true, true, true) => true,
        (true, true, false) => false,
        (true, false, false) => true,
        _ => false,
    }
}
