use std::collections::HashMap;

pub fn top(map: HashMap<String, usize>, n: usize) -> Vec<String> {
    let mut origin: Vec<_> = map.iter().collect();
    origin.sort_by(|a, b| b.1.cmp(a.1));

    origin
        .into_iter()
        .take(n)
        .map(|(key, _)| key.clone())
        .collect()
}

pub fn bot(map: HashMap<String, usize>, n: usize) -> Vec<String> {
    let mut origin: Vec<_> = map.iter().collect();
    origin.sort_by(|a, b| a.1.cmp(b.1));

    origin
        .into_iter()
        .take(n)
        .map(|(key, _)| key.clone())
        .collect()
}

pub fn print_result(vec: Vec<String>) {}
