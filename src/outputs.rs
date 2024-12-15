use std::{collections::HashMap, usize};

pub fn top(map: HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut origin: Vec<_> = map.iter().collect();
    origin.sort_by(|a, b| b.1.cmp(a.1));

    origin
        .into_iter()
        .take(n)
        .map(|(key, val)| (key.clone(), *val))
        .collect::<Vec<(String, usize)>>()
}

pub fn bot(map: HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut origin: Vec<_> = map.iter().collect();
    origin.sort_by(|a, b| a.1.cmp(b.1));

    origin
        .into_iter()
        .take(n)
        .map(|(key, val)| (key.clone(), *val))
        .collect::<Vec<(String, usize)>>()
}
