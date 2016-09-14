extern crate core;

use std::collections::HashSet;
use std::hash::Hash;
use std::cmp::{min, max};

const MAX_PERCENT_DIST: usize = 1_000_000;

fn run_func<T>(func: &Fn(Vec<T>, Vec<T>) -> usize, first: Vec<T>, second: Vec<T>, minimum: usize, maximum: usize) -> usize {
    let len1 = first.len();
    let len2 = second.len();

    if len1 == 0 && len2 == 0 {
        return minimum;
    }

    if len1 == 0 || len2 == 0 {
        return maximum;
    }

    return func(first, second);
}

pub fn jaccard_dist<T>(first: Vec<T>, second: Vec<T>) -> usize where T: Eq + Hash {
    let func = |fir: Vec<T>, sec: Vec<T>| -> usize {
        let set1: HashSet<T> = fir.into_iter().collect();
        let set2: HashSet<T> = sec.into_iter().collect();

        let intersect_set = set1.intersection(&set2);
        let union_set = set1.union(&set2);

        let intersect = intersect_set.count();
        let union = union_set.count();

        MAX_PERCENT_DIST - (MAX_PERCENT_DIST * intersect) / union

    };

    return run_func(&func, first, second, 0, MAX_PERCENT_DIST);
}

pub fn modified_jaccard_dist<T: Eq>(first: Vec<T>, second: Vec<T>) -> usize where Vec<T>: Clone {
    let func = |fir: Vec<T>, sec: Vec<T>| -> usize {
        let len1 = fir.len();
        let mut sec_copy = sec.clone();

        let mut intersect = 0;
        let mut union = len1 + sec_copy.len();

        for i in 0..len1 {
            let len2 = sec_copy.len();
            for j in 0..len2 {
                if fir[i] == sec_copy[j] {
                    intersect += 1;
                    union -= 1;

                    // Swap remove doesn't preseve ordering, but computes in O(1) time.
                    sec_copy.swap_remove(j);
                    break;
                }
            }

        }

        return MAX_PERCENT_DIST - (MAX_PERCENT_DIST * intersect) / union;
    };

    return run_func(&func, first, second, 0, MAX_PERCENT_DIST);
}

pub fn levenshtein_dist<T: Eq>(first: Vec<T>, second: Vec<T>) -> usize {
    let first_len: usize = first.len();
    let second_len: usize = second.len();

    if first_len == 0 {
        return second_len;
    }

    if second_len == 0 {
        return first_len;
    }

    let mut dist = vec![vec![0; second_len + 1]; first_len + 1];

    for i in 0..first_len + 1 {
        dist[i][0] = i as usize;
    }

    for i in 0..second_len + 1 {
        dist[0][i] = i as usize;
    }

    for i in 1..first_len + 1 {
        for j in 1..second_len + 1 {
            let mut samezies = 1;

            if first[i - 1] == second[j - 1] {
                samezies = 0;
            }

            dist[i][j] = min(min(dist[i - 1][j] + 1, dist[i][j - 1] + 1), dist[i - 1][j - 1] + samezies);
        }
    }

    return dist[first_len][second_len];
}

pub fn hamming_dist<T: Eq>(first: Vec<T>, second: Vec<T>) -> usize {
    let func = |fir: Vec<T>, sec: Vec<T>| -> usize {
        let len1 = fir.len();
        let len2 = sec.len();

        if len1 != len2 {
            return max(len1, len2);
        }

        let mut dist: usize = 0;

        for i in 0..len1 {
            if fir[i] != sec[i] {
                dist += 1;
            }
        }

        return dist;
    };

    let len1 = first.len();
    let len2 = second.len();

    return run_func(&func, first, second, 0, max(len1, len2));
}

#[allow(dead_code)]
fn function<T: Eq + Hash>(_a: Vec<T>, _b: Vec<T>) -> usize {
    return 0;
}

#[allow(dead_code)]
fn convert_str<T: ToString>(word: T) -> Vec<char> {
    return word.to_string().chars().collect();
}

#[allow(dead_code)]
fn convert_hex(word: String) -> Vec<u32> {
    let mut ret = vec![];

    let hex_char = |hash: char| -> Option<u32> {
        hash.to_digit(16)
    };

    let to_bin = |hex1: u32| -> Vec<u32> {
        let mut bin = vec![];

        for i in 0..8 {
            let val: u32 = hex1 >> i & 1;
            bin.push(val);
        }

        bin
    };

    for chr in word.chars() {
        match hex_char(chr) {
            Some(hex_chr) => {
                ret.extend(to_bin(hex_chr));
            },
            None => {}
        }
    }

    return ret;
}

#[test]
fn convert_str_test() {
    assert_eq!(convert_str("johndoe1").len(), 8);
    assert_eq!(convert_str("johndoe\u{263a}").len(), 8);
}

#[test]
fn convert_hex_test() {
    assert_eq!(convert_hex("1".to_string()), [1, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(convert_hex("11".to_string()), [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn len_check_test() {
    let v1: Vec<String> = vec![];
    let v2: Vec<String> = vec![];
    let v_full: Vec<String> = vec!["foo".to_string()];

    assert_eq!(run_func(&function, v1.clone(), v2.clone(), 0, 1), 0);
    assert_eq!(run_func(&function, v_full.clone(), v2.clone(), 0, 1), 1);
    assert_eq!(run_func(&function, v1.clone(), v_full.clone(), 0, 1), 1);
}

#[test]
fn levenshtein_dist_test_samples() {
    assert_eq!(levenshtein_dist(convert_str("foo"), convert_str("food")), 1);
    assert_eq!(levenshtein_dist(convert_str("foo"), convert_str("bar")), 3);
    assert_eq!(levenshtein_dist(convert_str("foo"), convert_str("foe")), 1);
}

#[test]
fn levenshtein_dist_test_unicode() {
    assert_eq!(levenshtein_dist(convert_str("johndoe1"), convert_str("johndoe\u{263a}")), 1);
    assert_eq!(levenshtein_dist(convert_str("johndoe1"), convert_str("johndoe\u{263a}1")), 1);
    assert_eq!(levenshtein_dist(convert_str("johndoe1"), convert_str("johndoe\u{263a}\u{263a}")), 2);
    assert_eq!(levenshtein_dist(convert_str("johndoe\u{263a}"), convert_str("johndoe1")), 1);
}

#[test]
fn hamming_dist_test() {
    assert_eq!(hamming_dist(convert_str("foo"), convert_str("bar")), 3);
    assert_eq!(hamming_dist(convert_str(""), convert_str("")), 0);
    assert_eq!(hamming_dist(convert_str("foo"), convert_str("")), 3);
    assert_eq!(hamming_dist(convert_str(""), convert_str("bar")), 3);
    assert_eq!(hamming_dist(convert_str("foo"), convert_str("foe")), 1);
}

#[test]
fn jaccard_dist_test() {
    assert_eq!(jaccard_dist(convert_str("foo"), convert_str("bar")), 1_000_000);
    assert_eq!(jaccard_dist(convert_str("bar"), convert_str("ba")), 333_334);
    assert_eq!(jaccard_dist(convert_str("bar"), convert_str("baz")), 500_000);
    assert_eq!(jaccard_dist(convert_str("GG"), convert_str("GGGG")), 0);
    assert_eq!(jaccard_dist(convert_str("GGGG"), convert_str("GG")), 0);
    assert_eq!(jaccard_dist(convert_str("fooba 1234"), convert_str("fooba1234")), 111_112);
}

#[test]
fn modified_jaccard_dist_test() {
    assert_eq!(modified_jaccard_dist(convert_str("foo"), convert_str("bar")), 1_000_000);
    assert_eq!(modified_jaccard_dist(convert_str("bar"), convert_str("ba")), 333_334);
    assert_eq!(modified_jaccard_dist(convert_str("bar"), convert_str("baz")), 500_000);
    assert_eq!(modified_jaccard_dist(convert_str("GG"), convert_str("GGGG")), 500_000);
    assert_eq!(modified_jaccard_dist(convert_str("GGGG"), convert_str("GG")), 500_000);
    assert_eq!(modified_jaccard_dist(convert_str("fooba 1234"), convert_str("fooba1234")), 100_000);
}

#[test]
fn hamming_dist_test_edges() {
    assert_eq!(hamming_dist(convert_hex("".to_string()), convert_hex("".to_string())), 0);
    assert_eq!(hamming_dist(convert_hex("".to_string()), convert_hex("a".to_string())), 8);
    assert_eq!(hamming_dist(convert_hex("a".to_string()), convert_hex("".to_string())), 8);
    assert_eq!(hamming_dist(convert_hex("aa".to_string()), convert_hex("a".to_string())), 16);
}

#[test]
fn hamming_dist_test_sample() {
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("435e9db1634baca2".to_string())), 36);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("e13c832b7ce2720f".to_string())), 30);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("cd87c969b794125a".to_string())), 28);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("096d864c93b396b7".to_string())), 32);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("6dc3693d11d0da4b".to_string())), 20);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("4f6ad94847cd2539".to_string())), 34);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("33edac42c731b135".to_string())), 34);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("9327939737447c1c".to_string())), 34);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("5fa9e49021de9176".to_string())), 36);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("a991569a1a66ed99".to_string())), 30);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("eb90ed295a62b465".to_string())), 30);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("4d67581a3f97283c".to_string())), 36);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("87506bfe01a4f84f".to_string())), 14);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("9fade410215e517e".to_string())), 34);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("d7bbcb6c3a369040".to_string())), 28);
}

#[test]
fn hamming_dist_test_capital() {
    assert_eq!(hamming_dist(convert_hex("0590EB7E1129FA5B".to_string()), convert_hex("D7BBCB6C3A369040".to_string())), 28);
    assert_eq!(hamming_dist(convert_hex("0590EB7E1129FA5B".to_string()), convert_hex("d7bbcb6c3a369040".to_string())), 28);
    assert_eq!(hamming_dist(convert_hex("0590eb7e1129fa5b".to_string()), convert_hex("D7BBCB6C3A369040".to_string())), 28);
}
