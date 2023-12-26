#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if let Some(ilegal_number) = number.into_iter().find(|n| *n >= &from_base) {
        return Err(Error::InvalidDigit(*ilegal_number));
    }
    let mut number_base_10 = number
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, n)| n * from_base.pow(i as u32))
        .sum::<u32>();
    let mut result = vec![];
    while to_base <= number_base_10 {
        let n = number_base_10 % to_base;
        result.insert(0, n);
        number_base_10 = (number_base_10 - n) / to_base;
    }
    result.insert(0, number_base_10);
    Ok(result)
}
