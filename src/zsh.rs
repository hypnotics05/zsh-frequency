// TODO: Unhandled edge cases
//
// ";WORD" will be matched even if it's inline
// 150 1
//

use std::{collections::HashMap, fs::File, io::Read};

use regex::Regex;

/// Returns a hashmap of all programs names and the number of time they where found
pub fn map(mut file: File) -> HashMap<String, usize> {
    let base = Regex::new(r"(;|\||\\\n|&)\s*[\w.\\\/~]*[\w]").unwrap();
    let sudo = Regex::new(r"(;|\||\\\n|&)\s*sudo\s*[\w.\\\/~]*[\w]").unwrap();
    let mut map: HashMap<String, usize> = HashMap::new();

    let mut buf = Vec::new();
    match file.read_to_end(&mut buf) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to read from file: {}", e);
        }
    }
    let line = String::from_utf8_lossy(&buf).to_string();

    let mut store = find_base_sudo(&line, &base, &sudo);

    let recursive_str: String = sanitize_recursive_data(&line)
        .into_iter()
        .map(|s| {
            let mut ret = String::from(";");
            ret.push_str(s.as_str());
            ret.push_str("\n");
            ret
        })
        .collect();

    let mut recursive_vec = find_base_sudo(&recursive_str, &base, &sudo);
    store.append(&mut recursive_vec);

    store.into_iter().for_each(|s| {
        let val = map.entry(s).or_insert(0);
        *val += 1;
    });

    map
}

fn find_base_sudo(line: &String, base: &Regex, sudo: &Regex) -> Vec<String> {
    /*
     * Get all base casses out from string, matches any word starting with a semicolon and any
     * amount of space and ends with more space, a semicolon or is at the end of it's line.
     *
     * Check for any occurences of sudo, if there are any then use the sudo expression to extract
     * them.
     *
     */

    let mut store: Vec<String> = base
        .find_iter(line.as_str())
        .map(|s| {
            let mut matched = String::from(s.as_str());
            matched.remove(0);
            matched.trim().to_string()
        })
        .collect();

    sudo.find_iter(line.as_str()).for_each(|s| {
        let mut matched = String::from(s.as_str());
        matched.remove(0);
        let mut sudo = matched.trim().split(' ');
        store.push(sudo.next().unwrap().to_string());
        store.push(sudo.next().unwrap().to_string());
    });

    store
}

fn sanitize_recursive_data(line: &String) -> Vec<String> {
    let expansion = Regex::new(r"\$\(.+\)").unwrap();

    expansion
        .find_iter(line.as_str())
        .map(|s| {
            let binding = String::from(s.as_str());
            let mut matched = binding.chars();
            matched.next();
            matched.next();
            matched.next_back();
            matched.as_str().trim().to_string()
        })
        .collect()
}
