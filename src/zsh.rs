const SKIP_RANGE: u8 = 15; // How many chars to skip to get to cmd

/*
 * Example of the line that we need to parse
 * : 1729482722:0;systemctl hybrid-sleep
 * : 1729543437:0;eval $(ssh-agent)
 */

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Bytes, Read},
};

// FIXME: hash map is missing most values, why?
pub fn gen_hash_map(file: File) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut prog: Vec<u8> = Vec::new(); // here is where we collect
    let mut iter = BufReader::with_capacity(file.metadata().unwrap().len() as usize, file).bytes();
    let mut more: bool = true;
    let mut bin: String;
    while more {
        more = collector(&mut iter, &mut prog);
        bin = String::from_utf8(prog.clone()).unwrap();
        let key = map.entry(bin).or_insert(1);
        *key += 1;
    }
    map
}

// keeping this as seperate function for now, incase we want to expand functionality at some point
fn collector(iter: &mut Bytes<BufReader<File>>, prog: &mut Vec<u8>) -> bool {
    // skips ": NUM:0;"
    for _ in 0..SKIP_RANGE {
        iter.next();
    }
    // tries to build string
    loop {
        match iter.next() {
            Some(x) => match x {
                Ok(y) => {
                    if y != ' ' as u8 {
                        prog.push(y);
                    } else {
                        break;
                    }
                }
                _ => continue, // catching any error here because Err doesn't work?
            },
            // Might not be nescesary as if this where to occur, the file is missformatted
            // Should we panic here?
            None => return false, // exit function, no more file to read
        }
    }
    let mut str1: u8 = ' ' as u8;
    let mut str2: u8;
    loop {
        match iter.next() {
            Some(x) => match x {
                Ok(x) => {
                    str2 = str1;
                    str1 = x;
                    if str1 == '\n' as u8 && str2 == ':' as u8 {
                        return true;
                    }
                }
                _ => {
                    str2 = str1; // still don't know how to fix this
                    str1 = ' ' as u8;
                }
            },
            None => return false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{path::Path, usize};

    fn build_default_map() -> HashMap<String, usize> {
        HashMap::from([
            (String::from("ll"), 3),
            (String::from("systemctl"), 4),
            (String::from("grep"), 2),
            (String::from("free"), 1),
            (String::from("Hyprland"), 1),
            (String::from("mpv"), 1),
            (String::from("eval"), 1),
            (String::from("cd"), 1),
        ])
    }

    fn iterator(path: &str) -> Bytes<BufReader<File>> {
        let file = open_file(path);
        BufReader::with_capacity(file.metadata().unwrap().len() as usize, file).bytes()
    }

    fn open_file(path: &str) -> File {
        match File::open(Path::new(&path)) {
            Err(err) => panic!("Couldn't open {}: {}", path, err),
            Ok(file) => return file,
        }
    }

    fn vec_u8_from_str(target: &str) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        for c in target.chars() {
            vec.push(c as u8);
        }
        vec
    }

    #[test]
    fn test_collector_build_single() {
        let mut prog: Vec<u8> = Vec::new();
        let mut iter = iterator("test/collector-build-single.log");
        let _ = collector(&mut iter, &mut prog);
        assert_eq!(prog, vec_u8_from_str("systemctl"));
    }

    // Failing test, only "systemcl" is being read, and only twice.
    #[test]
    fn test_collector_build_map() {
        assert_eq!(
            gen_hash_map(open_file("test/collector-build-map.log")),
            build_default_map()
        );
    }

    // tests if skip section can infact skip past utf-16 chars
    #[test]
    fn test_collector_build_map_utf_16() {}
}
