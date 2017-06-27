
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str;
extern crate smallstring;
use smallstring::SmallString;


pub fn words_list(path: &str) -> Vec<SmallString> {
    let f = File::open(path).unwrap();
    let file = BufReader::new(&f);

    file.lines().map(|x| {
        let stack: SmallString = x.unwrap().into();
        stack
    }).collect()
}

