#![feature(test)]
mod bkstring;
mod bknode;
mod bkdist;

extern crate rand;
extern crate test;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use std::path::Path;

fn lines_from_file<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>, io::Error>
where P: AsRef<Path> {
    let file = try!(File::open(filename));
    Ok(BufReader::new(file).lines())
}


fn main() {
}
