use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.into_iter().fold(BTreeMap::new(), |map, (key, letters)| {
        letters.into_iter().fold(map, |mut sub_map, letter| {
            sub_map.insert(letter.to_ascii_lowercase(), *key);
            sub_map
        })
    })
}
