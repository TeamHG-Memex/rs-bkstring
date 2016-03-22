use std::cmp::min;

const MAX_PERCENT_DIST: usize = 100;
const MAX_HEX_HAM_DIST: usize = 256;

pub fn mod_j_dist(first: String, second: String) -> usize {
    let len1 = first.chars().count();
    let mut sec_copy = second.clone().to_string();
    let len2 = sec_copy.chars().count();


    if len1 == 0 || len2 == 0 {
        return MAX_PERCENT_DIST;
    }

    let mut intersect = 0;
    let mut union = len1 + len2;

    for i in 0..len1 {
        match sec_copy.find(first.chars().nth(i).unwrap()) {
            Some(idx) => {
                intersect += 1;
                union -= 1;
                sec_copy.remove(idx);
            },
            None => {}
        }
    }

    return MAX_PERCENT_DIST - (MAX_PERCENT_DIST * intersect) / union;
}

pub fn l_dist(first: String, second: String) -> usize {
    let first_len: usize = first.chars().count();
    let second_len: usize = second.chars().count();

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

            if first.chars().nth(i - 1) == second.chars().nth(j - 1) {
                samezies = 0;
            }

            dist[i][j] = min(min(dist[i - 1][j] + 1, dist[i][j - 1] + 1), dist[i - 1][j - 1] + samezies);
        }
    }

    return dist[first_len][second_len];
}

pub fn jaro_dist(first: String, second: String) -> usize {
    let len1 = first.chars().count();
    let len2 = second.chars().count();

    // If both strings are empty, they are both exact matches for eachother.
    if first.is_empty() && second.is_empty() {
        return 0;
    }

    // If either string is empty, there will be no matches 'm', so similarity is defined as 0,
    // therefore distance will be maximized.
    if first.is_empty() || second.is_empty() {
        return MAX_PERCENT_DIST;
    }

    // This implementation assumes first is longer than second.  So in the case where
    // first is shorter, we will return the reversed distance.
    if len1 < len2 {
        return jaro_dist(second, first);
    }

    // The mod corrects our half search for even vs odd lengths.
    let half = len1 / 2 + len1 % 2;
    let mut matches = vec![];

    for i in 0..len1 {
        for j in 0..half {
            let sum = i + j;

            if i + j < len2 &&
            second.chars().nth(sum) == first.chars().nth(i) &&
            !matches.contains(&sum) {
                matches.push(sum);
            }

            if i >= j {
                let diff = i - j;

                if i - j < len2 &&
                second.chars().nth(diff) == first.chars().nth(i) &&
                !matches.contains(&diff) {
                    matches.push(diff);
                }
            }
        }
    }

    let m = matches.len();

    // Jaro similarity is defined as 0 for m == 0, so we return max distance here.
    if m == 0 {
        return MAX_PERCENT_DIST;
    }

    let mut t = 0;

    for i in 0..m - 1 {
        if matches[i] > matches[i + 1] {
            t += 1;
        }
    }

    // These parts are very simple, but broken out because we have to do a lot of
    // casting.

    // We are effectively reproducing the Jaro Distance formula:
    //     100 * (1 - 1/3 * (m / len1 + m / len2 + (m - t) / m))
    let fir = m as f64 / len1 as f64;
    let sec = m as f64 / len2 as f64;
    let transpose = (m as f64 - t as f64) / m as f64;

    let form = (fir + sec + transpose) / 3f64;
    let dist = (1f64 - form) * MAX_PERCENT_DIST as f64;

    // We return ceil to make this act the same as the C implementation of casting from float to usize.
    return dist.ceil() as usize;
}

pub fn hex_ham_dist(first: String, second: String) -> usize {
    if first.is_empty() || second.is_empty() {
        return MAX_HEX_HAM_DIST;
    }

    if first.len() != second.len() {
        return MAX_HEX_HAM_DIST;
    }

    //
    let hex_char = |hash: String, idx: usize| -> Option<u32> {
        hash.chars().nth(idx).unwrap().to_digit(16)
    };

    let sum_xor = |hex1: u32, hex2: u32| -> usize {
        let mut sum: usize = 0;

        for i in 0..8 {
            if hex1 >> i & 1 != hex2 >> i & 1 {
                sum += 1;
            }
        }

        sum
    };

    let mut sum: usize = 0;

    for i in 0..first.chars().count() {
        match hex_char(first.clone(), i) {
            Some(hex1) => {
                match hex_char(second.clone(), i) {
                    Some(hex2) => {
                        sum += sum_xor(hex1, hex2);
                    },
                    None => {}
                }
            },
            None => {}
        }
    }

    return sum;
}

#[test]
fn l_dist_test_samples() {
    assert_eq!(l_dist("foo".to_string(), "food".to_string()), 1);
    assert_eq!(l_dist("foo".to_string(), "bar".to_string()), 3);
    assert_eq!(l_dist("foo".to_string(), "foe".to_string()), 1);
}

#[test]
fn l_dist_test_unicode() {
    assert_eq!(l_dist("johndoe1".to_string(), "johndoe\u{263a}".to_string()), 1);
    assert_eq!(l_dist("johndoe1".to_string(), "johndoe\u{263a}1".to_string()), 1);
    assert_eq!(l_dist("johndoe1".to_string(), "johndoe\u{263a}\u{263a}".to_string()), 2);
    assert_eq!(l_dist("johndoe\u{263a}".to_string(), "johndoe1".to_string()), 1);
}

#[test]
fn mod_j_dist_test() {
    assert_eq!(mod_j_dist("foo".to_string(), "bar".to_string()), 100);
    assert_eq!(mod_j_dist("bar".to_string(), "ba".to_string()), 34);
    assert_eq!(mod_j_dist("bar".to_string(), "baz".to_string()), 50);
    assert_eq!(mod_j_dist("GG".to_string(), "GGGG".to_string()), 50);
    assert_eq!(mod_j_dist("GGGG".to_string(), "GG".to_string()), 50);
    assert_eq!(mod_j_dist("fooba 1234".to_string(), "fooba1234".to_string()), 10);
}

#[test]
fn jaro_dist_test_edges() {
    // Test maximum dist
    assert_eq!(jaro_dist("foo".to_string(), "foo".to_string()), 0);
    assert_eq!(jaro_dist("123456".to_string(), "456123".to_string()), 100);
    assert_eq!(jaro_dist("".to_string(), "".to_string()), 0);
    assert_eq!(jaro_dist("".to_string(), "foo".to_string()), 100);
}

#[test]
fn jaro_dist_test_reversible() {
    // Test communtative property of Jaro Distance
    assert_eq!(jaro_dist("longererererer".to_string(), "shorter".to_string()), jaro_dist("shorter".to_string(), "longererererer".to_string()));
}

#[test]
fn jaro_dist_test_sample() {
    // Test some sample scores
    assert_eq!(jaro_dist("duane".to_string(), "dwayne".to_string()), 18);
    assert_eq!(jaro_dist("martha".to_string(), "marhta".to_string()), 6);
    assert_eq!(jaro_dist("dixon".to_string(), "dicksonx".to_string()), 24);
    assert_eq!(jaro_dist("1234567@".to_string(), "12354@".to_string()), 14);
}

#[test]
fn hex_ham_dist_test_edges() {
    assert_eq!(hex_ham_dist("".to_string(), "".to_string()), MAX_HEX_HAM_DIST);
    assert_eq!(hex_ham_dist("".to_string(), "a".to_string()), MAX_HEX_HAM_DIST);
    assert_eq!(hex_ham_dist("a".to_string(), "".to_string()), MAX_HEX_HAM_DIST);
    assert_eq!(hex_ham_dist("aa".to_string(), "a".to_string()), MAX_HEX_HAM_DIST);
}

#[test]
fn hex_ham_dist_test_sample() {
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "435e9db1634baca2".to_string()), 36);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "e13c832b7ce2720f".to_string()), 30);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "cd87c969b794125a".to_string()), 28);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "096d864c93b396b7".to_string()), 32);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "6dc3693d11d0da4b".to_string()), 20);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "4f6ad94847cd2539".to_string()), 34);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "33edac42c731b135".to_string()), 34);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "9327939737447c1c".to_string()), 34);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "5fa9e49021de9176".to_string()), 36);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "a991569a1a66ed99".to_string()), 30);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "eb90ed295a62b465".to_string()), 30);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "4d67581a3f97283c".to_string()), 36);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "87506bfe01a4f84f".to_string()), 14);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "9fade410215e517e".to_string()), 34);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "d7bbcb6c3a369040".to_string()), 28);
}

#[test]
fn hex_ham_dist_test_capital() {
    assert_eq!(hex_ham_dist("0590EB7E1129FA5B".to_string(), "D7BBCB6C3A369040".to_string()), 28);
    assert_eq!(hex_ham_dist("0590EB7E1129FA5B".to_string(), "d7bbcb6c3a369040".to_string()), 28);
    assert_eq!(hex_ham_dist("0590eb7e1129fa5b".to_string(), "D7BBCB6C3A369040".to_string()), 28);
}
