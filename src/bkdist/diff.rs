use std::cmp::{min, max};

const MAX_PERCENT_DIST: usize = 100;

pub fn word_len(word: String) -> usize {
    return word.chars().count();
}

pub fn sentence_len(sentence: String) -> usize {
    let ret: Vec<&str> = sentence.split(" ").collect();
    return ret.len();
}


pub fn j_diff(length: usize, key: usize, dist: usize) -> usize {
    let more = max(length, key);
    let less = min(length, key);

    if more + dist == 0 {
        return 0;
    }

    min(MAX_PERCENT_DIST - (MAX_PERCENT_DIST as f64 * (less - dist) as f64 / (more + dist) as f64) as usize, MAX_PERCENT_DIST)
}

pub fn l_diff(length: usize, key: usize, dist: usize) -> usize {
    if length > key {
        return length - key + dist;
    }

    key - length + dist
}

pub fn jaro_diff(length: usize, key: usize, dist: usize) -> usize {
    let more = max(length, key);
    let less = min(length, key);

    if more == 0 {
        return dist;
    }

    dist + (dist as f64 / 3f64 * less as f64 / more as f64) as usize
}

#[allow(unused)]
pub fn hex_ham_diff(length: usize, key: usize, dist: usize) -> usize {
    dist
}
