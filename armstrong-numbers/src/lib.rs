pub fn is_armstrong_number(num: u32) -> bool {
    let mut candidate: u32 = 0;
    let number_string = num.to_string();
    let power = number_string.len() as u32;
    // for base in number_string.chars() {
    //     let number_to_add = match base.to_digit(10) {
    //         Some(it) => it.pow(power),
    //         None => return false,
    //     };
    //     candidate = match candidate.checked_add(number_to_add) {
    //         Some(it) => it,
    //         None => return false,
    //     }
    // }

    for base in number_string.chars() {
        candidate = match base.to_digit(10) {
            Some(it) => match candidate.checked_add(it.pow(power)) {
                Some(it) => it,
                None => return false,
            },
            None => return false,
        };
    }
    if candidate == num || num == 0 {
        return true;
    }
    false
}
