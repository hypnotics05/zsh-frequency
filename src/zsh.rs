#[allow(dead_code)]
const SKIP_RANGE: u8 = 14; // How many chars to skip to get to cmd

// TODO: add support for sudo, both by reading command after sudo and counting the number of sudo
// TODO: Add support to read inline commands

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Bytes, Read},
};

#[allow(dead_code)]
pub fn gen_hash_map(file: File) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut prog: Vec<u8> = Vec::new(); // here is where we collect
    let mut iter = BufReader::with_capacity(file.metadata().unwrap().len() as usize, file).bytes();
    let mut more: bool = true;
    let mut bin: String;
    iter.next();
    while more {
        more = collector(&mut iter, &mut prog);
        bin = String::from_utf8(prog.clone()).unwrap();
        let key = map.entry(bin).or_insert(0);
        *key += 1;
        let _ = &mut prog.clear();
    }
    map
}

#[allow(dead_code)]
fn collector(iter: &mut Bytes<BufReader<File>>, prog: &mut Vec<u8>) -> bool {
    for _ in 0..SKIP_RANGE {
        iter.next();
    }
    let mut str1: u8 = 0;
    let mut str2: u8;
    // tries to build string
    loop {
        match iter.next() {
            Some(x) => match x {
                Ok(y) => {
                    if y != ' ' as u8 {
                        if y == '\n' as u8 {
                            str1 = y;
                            break;
                        }
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
    loop {
        match iter.next() {
            Some(x) => match x {
                Ok(x) => {
                    str2 = str1;
                    str1 = x;
                    // HACK: debug statment to determine what the stream contains
                    // println!("Reading: 1 - {} && 2 - {}", str1 as char, str2 as char);
                    if str1 == ':' as u8 && str2 == '\n' as u8 {
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

    fn file_iterator(path: &str) -> Bytes<BufReader<File>> {
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
        let mut iter = file_iterator("tests/collector-build-single");
        iter.next();
        let _ = collector(&mut iter, &mut prog);
        assert_eq!(prog, vec_u8_from_str("systemctl"));
    }

    #[test]
    fn test_collector_multi() {
        let mut first: Vec<u8> = Vec::new();
        let mut second: Vec<u8> = Vec::new();
        let mut iter = file_iterator("tests/collector-2-strings");
        iter.next();
        let _ = collector(&mut iter, &mut first);
        let _ = collector(&mut iter, &mut second);

        assert_eq!(first, vec_u8_from_str("systemctl"));
        assert_eq!(second, vec_u8_from_str("grep"));
    }
}
