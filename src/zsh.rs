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
    for _ in 0..=SKIP_RANGE {
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
