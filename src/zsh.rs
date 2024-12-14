#[allow(dead_code)]
const SKIP_RANGE: u8 = 14; // How many chars to skip to get to cmd

// TODO: add support for sudo, both by reading command after sudo and counting the number of sudo
// TODO: Add support to read inline commands

use std::{collections::HashMap, fs::File, io::Read};

use regex::Regex;

#[allow(dead_code)]
pub fn map(mut file: File) -> HashMap<String, usize> {
    let base = Regex::new(r";\s*[^ \n]+").unwrap();
    let sudo = Regex::new(r";\s*sudo\s*[^ \n]+").unwrap();
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut line: String = String::new();
    let _ = file.read_to_string(&mut line);

    let mut store = collect(line, base, sudo);
    store.into_iter().for_each(|s| {
        let val = map.entry(s).or_insert(0);
        *val += 1;
    });

    // TODO: Read in nested expressions from $(expr)

    // FIXME: I think large files are not supported? Should break it up in smaller strings

    map
}

// REF: use regex instead of manually parsing
fn collect(line: String, base: Regex, sudo: Regex) -> Vec<String> {
    /*
     * Get all base casses out from string, matches any word starting with a semicolon and any
     * amount of space and ends with more space, a semicolon or is at the end of it's line.
     *
     * Check for any occurences of sudo, if there are any then use the sudo expression to extract
     * them.
     *
     * Then we find all text trapped in $() expressions, and re run them through the collector
     */
    let mut store = Vec::new();

    base.find_iter(line.as_str()).for_each(|s| {
        let mut matched = String::from(s.as_str());
        matched.remove(0);
        store.push(matched.trim().to_string());
    });

    sudo.find_iter(line.as_str()).for_each(|s| {
        let mut matched = String::from(s.as_str());
        matched.remove(0);
        let mut sudo = matched.trim().split(' ');
        store.push(sudo.next().unwrap().to_string());
        store.push(sudo.next().unwrap().to_string());
    });

    store
}
