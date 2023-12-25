use num::BigInt;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(1..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    ((0..a).into_iter().fold(g, |acc, i| {
        if acc * i % p == 0 {
            acc * i / p
        } else {
            acc * i
        }
    }) % p)
    // .to_string()
    // .parse::<u64>()
    // .unwrap()
    // g.pow(a as u32) % p
}
//A = gᵃ mod p

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    ((0..a).into_iter().fold(b_pub, |acc, i| {
        if acc * i % p == 0 {
            acc * i / p
        } else {
            acc * i
        }
    }) % p)
    // .to_string()
    // .parse::<u64>()
    // .unwrap()
    // b_pub.pow(a as u32) % p
}

//s = Bᵃ mod p
