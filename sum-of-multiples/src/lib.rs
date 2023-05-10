use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .into_iter()
        .fold(HashSet::new(), |acc, f| add_multiples(acc, &limit, *f))
        .into_iter()
        .sum()
}

fn add_multiples(mut set: HashSet<u32>, limit: &u32, factor: u32) -> HashSet<u32> {
    if factor == 0 {
        return set;
    }
    let mut next_multiple = factor;
    while &next_multiple < limit {
        set.insert(next_multiple);
        next_multiple += factor;
    }
    set
}
