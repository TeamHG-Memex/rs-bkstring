extern crate bkstring;

#[cfg(bkdist_tests)]
mod bkdist_tests {

use bkstring::bkdist::{levenshtein_dist, modified_jaccard_dist, jaro_dist, hex_ham_dist};


#[test]
fn levenshtein_dist_test_samples() {
    assert_eq!(levenshtein_dist("foo".to_string(), "food".to_string()), 1);
    assert_eq!(levenshtein_dist("foo".to_string(), "bar".to_string()), 3);
    assert_eq!(levenshtein_dist("foo".to_string(), "foe".to_string()), 1);
}

#[test]
fn levenshtein_dist_test_unicode() {
    assert_eq!(levenshtein_dist("johndoe1".to_string(), "johndoe\u{263a}".to_string()), 1);
    assert_eq!(levenshtein_dist("johndoe1".to_string(), "johndoe\u{263a}1".to_string()), 1);
    assert_eq!(levenshtein_dist("johndoe1".to_string(), "johndoe\u{263a}\u{263a}".to_string()), 2);
    assert_eq!(levenshtein_dist("johndoe\u{263a}".to_string(), "johndoe1".to_string()), 1);
}

#[test]
fn modified_jaccard_dist_test() {
    assert_eq!(modified_jaccard_dist("foo".to_string(), "bar".to_string()), 100);
    assert_eq!(modified_jaccard_dist("bar".to_string(), "ba".to_string()), 34);
    assert_eq!(modified_jaccard_dist("bar".to_string(), "baz".to_string()), 50);
    assert_eq!(modified_jaccard_dist("GG".to_string(), "GGGG".to_string()), 50);
    assert_eq!(modified_jaccard_dist("GGGG".to_string(), "GG".to_string()), 50);
    assert_eq!(modified_jaccard_dist("fooba 1234".to_string(), "fooba1234".to_string()), 10);
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
}
