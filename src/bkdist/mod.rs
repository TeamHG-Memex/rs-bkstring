mod char_function;
mod diff;
mod word_function;

use self::char_function::*;
use self::diff::*;
use self::word_function::*;

#[derive(Clone)]
pub struct _Metric {
    pub function: fn(String, String) -> usize,
    pub diff: fn(usize, usize, usize) -> usize,
    pub len: fn(String) -> usize
}

impl _Metric {
    pub extern fn jac_dist() -> _Metric {
        _Metric {
            function: jac_dist,
            diff: j_diff,
            len: word_len
        }
    }

    pub extern fn mod_j_dist() -> _Metric {
        _Metric {
            function: mod_j_dist,
            diff: j_diff,
            len: word_len
        }
    }

    pub extern fn l_dist() -> _Metric {
        _Metric {
            function: l_dist,
            diff: l_diff,
            len: word_len
        }
    }

    pub extern fn jaro_dist() -> _Metric {
        _Metric {
            function: jaro_dist,
            diff: jaro_diff,
            len: word_len
        }
    }

    pub extern fn hex_ham_dist() -> _Metric {
        _Metric {
            function: hex_ham_dist,
            diff: hex_ham_diff,
            len: word_len
        }
    }

    pub extern fn word_bigram_j_dist() -> _Metric {
        _Metric {
            function: word_bigram_j_dist,
            diff: j_diff,
            len: sentence_len
        }
    }

    pub extern fn word_l_dist() -> _Metric {
        _Metric {
            function: word_l_dist,
            diff: l_diff,
            len: sentence_len
        }
    }

    pub extern fn word_fuzzy_l_dist() -> _Metric {
        _Metric {
            function: word_fuzzy_l_dist,
            diff: l_diff,
            len: sentence_len
        }
    }
}

pub type Metric = _Metric;

#[test]
fn make_jac_dist() {
    let foo = Metric::jac_dist();
    assert_eq!((foo.function)("".to_string(), "".to_string()), 0);
    assert_eq!((foo.diff)(0, 0, 0), 0);
}

#[test]
fn make_mod_j_dist() {
    let foo = Metric::mod_j_dist();
    assert_eq!((foo.function)("".to_string(), "".to_string()), 0);
    assert_eq!((foo.diff)(0, 0, 0), 0);
}

#[test]
fn make_l_dist() {
    let foo = Metric::l_dist();
    assert_eq!((foo.function)("".to_string(), "".to_string()), 0);
    assert_eq!((foo.diff)(0, 0, 0), 0);
}

#[test]
fn make_jaro_dist() {
    let foo = Metric::jaro_dist();
    assert_eq!((foo.function)("".to_string(), "".to_string()), 0);
    assert_eq!((foo.diff)(0, 0, 0), 0);
}

#[test]
fn make_hex_ham_dist_dist() {
    let foo = Metric::hex_ham_dist();
    assert_eq!((foo.function)("".to_string(), "".to_string()), 0);
    assert_eq!((foo.diff)(0, 0, 0), 0);
}

#[test]
fn make_word_bigram_j_dist() {
    let foo = Metric::word_bigram_j_dist();
    assert_eq!((foo.function)("".to_string(), "".to_string()), 0);
    assert_eq!((foo.diff)(0, 0, 0), 0);
}

#[test]
fn make_word_l_dist() {
    let foo = Metric::word_l_dist();
    assert_eq!((foo.function)("".to_string(), "".to_string()), 0);
    assert_eq!((foo.diff)(0, 0, 0), 0);
}

#[test]
fn make_word_fuzzy_l_dist() {
    let foo = Metric::word_fuzzy_l_dist();
    assert_eq!((foo.function)("".to_string(), "".to_string()), 0);
    assert_eq!((foo.diff)(0, 0, 0), 0);
}
