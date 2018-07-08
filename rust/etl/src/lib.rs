use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    input.iter()
        .flat_map(|(key, values)|
            values.iter()
                .map(|v| (v.to_ascii_lowercase(), *key) )
                .collect::<Vec<_>>()
        )
        .collect()
}
