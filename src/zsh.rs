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

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
};

pub struct Zsh {
    collection: HashMap<String, usize>,
}

impl Zsh {
    pub fn new(file: File) -> Self {
        Self {
            collection: {
                let mut map = HashMap::new();
                let mut col: Vec<u8> = Vec::new();
                let buffer =
                    BufReader::with_capacity(file.metadata().unwrap().len() as usize, file);
                map
            },
        }
    }
}
