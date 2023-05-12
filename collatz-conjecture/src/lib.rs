pub fn collatz(n: u64) -> Option<u64> {
    fn _collatz(n: u64, i: u64) -> Option<u64> {
        match n {
            0 => None,
            1 => Some(i),
            _ => match n % 2 {
                0 => _collatz(n / 2, i + 1),
                _ => n
                    .checked_mul(3)?
                    .checked_add(1)
                    .and_then(|n| _collatz(n, i + 1)),
            },
        }
    }
    _collatz(n, 0)
}
