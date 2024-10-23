// Here is where the struct, hashmap and all methods used for the analysis will go.
// there are invalid utf-8 characters in the file (japanese names of anime in mv operations) so
// we need a proper way to handle this, because none of the binaries contain invalid chars if
// we read in chars at a time and store them into a string, we can avoid reading in invalid
// chars. Then just do substring analysis to do the find and place in the HashMap

// A way to avoid non utf-8 chars is just to ingore it on err, but how?

// Use std::io::BufRead to read file? It is very good at making frequent small read calls,
// which we will be needing (every line is a read call), we could use peek() or an iterator
// with bytes()

// Creates the file buffer
// let buffer = BufReader::with_capacity(file.metadata().unwrap().len() as usize, file);

// Buffer should create an Iterator, with this we could call next(), which return a reference to
// the value, we could collect these references in some sort of collector (Vec<>?) while the
// ref is not ' ', then turn the collection to a string and place it into the HashMap. Then we need
// to next til '\n' then we can skip 15 chars with advance_by()

// Use BufReader::read_until(), ' ' and '\n' and ';'

// Still need a way to ignore u16 when iterating over BufReader()
// try using a match statment with Some(x) use x, with None we could skip, but because the iterator
// returns None when there are no more bytes to iterate, we should count consecutive Nones and if
// the None surpass a limit, we can exit.

const SKIP_RANGE: u8 = 14; // How many chars to skip to get to cmd

//const NONE_RATE: u8 = 255; // How many non UTF-8 chars we can meet before we give up on the file

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Bytes, Read},
};

pub fn genHashMap(file: File) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    let mut prog: Vec<u8> = Vec::new(); // here is where we collect
    let mut iter = BufReader::with_capacity(file.metadata().unwrap().len() as usize, file).bytes();

    // TODO: Add collector and skip function calls here
    iter.next(); // buffer to remove the ':' from the first case

    /*
     * Example of the line types that we need to parse
     * : 1729482722:0;systemctl hybrid-sleep
     * : 1729543437:0;eval $(ssh-agent)
     */
    map
}

fn collector(iter: &mut Bytes<BufReader<File>>, prog: &mut Vec<u8>) {
    for _ in [..SKIP_RANGE] {
        iter.next();
    }
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
                _ => continue, // catching any error here because Err doesn't work? Need to
                               // insvestigate more later.
            },
            None => break,
        }
    }
}
/*
 * Looping over iter until we meet the patern '\n:'
 */
fn skip(iter: &mut Bytes<BufReader<File>>) {
    let mut str1: u8 = ' ' as u8;
    let mut str2: u8;
    loop {
        match iter.next() {
            Some(x) => match x {
                Ok(x) => {
                    str2 = str1;
                    str1 = x;
                    if str1 == '\n' as u8 && str2 == ':' as u8 {
                        return;
                    }
                }
                _ => {
                    str2 = str1;
                    str1 = ' ' as u8;
                }
            },
            None => break,
        }
    }
}
