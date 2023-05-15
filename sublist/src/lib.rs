#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.eq(_second_list) {
        return Comparison::Equal;
    }
    if is_sublist(_first_list, _second_list) {
        return Comparison::Sublist;
    }
    if is_sublist(_second_list, _first_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    if _first_list.len() > _second_list.len() {
        return false;
    }
    if _first_list.len() == 0 {
        return true;
    }
    _second_list
        .windows(_first_list.len())
        .any(|w| w == _first_list)
}
