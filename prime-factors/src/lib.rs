pub fn factors(n: u64) -> Vec<u64> {
    (2..=n)
        .into_iter()
        .filter(|i| is_prime(*i) || n % i == 0)
        .collect()
}

fn is_prime(n: u64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i: u64 = 5;
    while i.pow(2) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return true;
        }
        i += 6;
    }
    true
}
