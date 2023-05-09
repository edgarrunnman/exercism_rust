pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    let mut i: u32 = 2;
    while primes.len() <= n as usize {
        if is_prime(i) {
            primes.push(i);
        }
        i += 1;
    }
    *primes.last().unwrap()
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f32).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
