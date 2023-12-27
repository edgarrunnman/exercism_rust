use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let split = input.split(" == ").collect::<Vec<_>>();
    let result_str = split.last().unwrap();
    let additions_str = split.first().unwrap().split(" + ").collect::<Vec<_>>();
    todo!("Solve the alphametic {input:?}")
}
