use std::cmp::Ordering::*;
pub fn find<L, T>(array: L, key: T) -> Option<usize>
where
    L: AsRef<[T]>,
    T: Ord,
{
    let array = array.as_ref();
    let half = array.len() / 2;
    match key.cmp(array.get(half)?) {
        Equal => Some(half),
        Greater => find(&array[half + 1..], key).map(|s| half + s + 1),
        Less => find(&array[..half], key),
    }
}
