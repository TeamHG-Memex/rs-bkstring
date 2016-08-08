use std::cmp::min;
use bkdist::char_function::l_dist;

const MAX_PERCENT_DIST: usize = 100;

fn word_ngrams(sentence: String, size: usize) -> Vec<String> {
    let split: Vec<&str> = sentence.split(" ").collect();
    let mut vec: Vec<String> = vec![];

    for words in split.windows(size) {
        let w: Vec<String> = vec![words[0].to_string(), words[1].to_string()];

        vec.push(w.join(" "));
    }

    return vec;
}

fn words(sentence: String) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    let split: Vec<&str> = sentence.split(" ").collect();

    for s in split {
        if s != "" {
            ret.push(s.to_string());
        }
    }

    return ret;
}

pub fn word_bigram_j_dist(first: String, second: String) -> usize {
    let mut vec1 = word_ngrams(first, 2);
    let mut vec2 = word_ngrams(second, 2);
    vec1.dedup();
    vec2.dedup();

    let len1 = vec1.len();
    let len2 = vec2.len();

    if vec1.is_empty() && vec2.is_empty() {
        return 0;
    }

    if vec1.is_empty() || vec2.is_empty() {
        return MAX_PERCENT_DIST;
    }

    let mut union = len1 + len2;
    let mut intersect = 0;
    for word in vec2 {
        if vec1.contains(&word) {
            union -= 1;
            intersect += 1;
        }
    }

    return 100 - (100 * intersect) / union;
}

pub fn word_l_dist(first: String, second: String) -> usize {
    let vec1 = words(first);
    let vec2 = words(second);
    let len1 = vec1.len();
    let len2 = vec2.len();

    if len1 == 0 {
        return len2;
    }

    if len2 == 0 {
        return len1;
    }

    let mut dist = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..len1 + 1 {
        dist[i][0] = i;
    }

    for i in 0..len2 + 1 {
        dist[0][i] = i;
    }

    for i in 1..len1 + 1 {
        for j in 1..len2 + 1 {
            let mut samezies = 1;

            if vec1[i - 1] == vec2[j - 1] {
                samezies = 0;
            }

            dist[i][j] = min(min(dist[i - 1][j] + 1, dist[i][j - 1] + 1), dist[i - 1][j - 1] + samezies);
         }
    }

    return dist[len1][len2];
}

pub fn word_fuzzy_l_dist(first: String, second: String) -> usize {
    let vec1 = words(first);
    let vec2 = words(second);
    let len1 = vec1.len();
    let len2 = vec2.len();

    if len1 == 0 {
        return len2;
    }

    if len2 == 0 {
        return len1;
    }

    let mut dist = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..len1 + 1 {
        dist[i][0] = i;
    }

    for i in 0..len2 + 1 {
        dist[0][i] = i;
    }

    for i in 1..len1 + 1 {
        for j in 1..len2 + 1 {
            let mut samezies = 1;

            if l_dist(vec1[i - 1].to_string(), vec2[j - 1].to_string()) <= 1 {
                samezies = 0;
            }

            dist[i][j] = min(min(dist[i - 1][j] + 1, dist[i][j - 1] + 1), dist[i - 1][j - 1] + samezies);
         }
    }
    println!("working!");

    return dist[len1][len2];
}

#[test]
fn test_word_bigram_j_dist_edges() {
    assert_eq!(word_bigram_j_dist("".to_string(), "".to_string()), 0);
    assert_eq!(word_bigram_j_dist("".to_string(), "fat lazy cat".to_string()), MAX_PERCENT_DIST);
    assert_eq!(word_bigram_j_dist("fat lazy cat".to_string(), "".to_string()), MAX_PERCENT_DIST);
}

#[test]
fn test_word_bigram_j_dist_sample() {
    assert_eq!(word_bigram_j_dist("fat lazy cat".to_string(), "fuzzy lazy cat".to_string()), 67);
    assert_eq!(word_bigram_j_dist("fat furry lazy cat".to_string(), "fluffy furry lazy cat".to_string()), 50);
}

#[test]
fn test_word_bigram_j_dist_reverse() {
    assert_eq!(word_bigram_j_dist("foo bar baz".to_string(), "baz bar foo".to_string()), MAX_PERCENT_DIST);
}

#[test]
fn test_word_bigram_j_dist_equal() {
    assert_eq!(word_bigram_j_dist("foo bar baz".to_string(), "foo bar baz".to_string()), 0);
}

#[test]
fn test_word_bigram_j_dist_dup() {
    assert_eq!(word_bigram_j_dist("foo foo foo bar baz".to_string(), "foo foo bar baz".to_string()), 0);
}

#[test]
fn test_word_l_dist_edges() {
    assert_eq!(word_l_dist("".to_string(), "".to_string()), 0);
    assert_eq!(word_l_dist("".to_string(), "fat lazy cat".to_string()), 3);
    assert_eq!(word_l_dist("fat lazy cat".to_string(), "".to_string()), 3);
}

#[test]
fn test_word_fuzzy_l_dist_edges() {
    assert_eq!(word_fuzzy_l_dist("".to_string(), "".to_string()), 0);
    assert_eq!(word_fuzzy_l_dist("".to_string(), "fat lazy cat".to_string()), 3);
    assert_eq!(word_fuzzy_l_dist("fat lazy cat".to_string(), "".to_string()), 3);
}

#[test]
fn test_word_fuzzy_l_dist_zeros() {
    assert_eq!(word_fuzzy_l_dist("foo bar baz".to_string(), "foo bar baz".to_string()), 0);
    assert_eq!(word_fuzzy_l_dist("food bar baz".to_string(), "foo bar baz".to_string()), 0);
}

#[test]
fn test_word_l_dist_zeros() {
    assert_eq!(word_l_dist("foo bar baz".to_string(), "foo bar baz".to_string()), 0);
}

#[test]
fn test_word_l_dist_reversible() {
    assert_eq!(word_l_dist("foo bar baz".to_string(), "the other thing".to_string()), word_l_dist("the other thing".to_string(), "foo bar baz".to_string()));
}

#[test]
fn test_word_fuzzy_l_dist_reversible() {
    assert_eq!(word_fuzzy_l_dist("foo bar baz".to_string(), "the other thing".to_string()), word_fuzzy_l_dist("the other thing".to_string(), "foo bar baz".to_string()));
}

#[test]
fn test_word_l_dist_sample() {
    assert_eq!(word_l_dist("fat lazy cat".to_string(), "fuzzy lazy cat".to_string()), 1);
    assert_eq!(word_l_dist("fat furry lazy cat".to_string(), "fluffy furry lazy cat".to_string()), 1);
}

#[test]
fn test_word_fuzzy_l_dist_sample() {
    assert_eq!(word_fuzzy_l_dist("fatt lazy cat".to_string(), "fuzzy lazy cat".to_string()), 1);
    assert_eq!(word_fuzzy_l_dist("fat furrry lazy cat".to_string(), "fluffy furry lazy cat".to_string()), 1);
    assert_eq!(word_fuzzy_l_dist("fat lazy cat".to_string(), "fuzzy lazy cat".to_string()), 1);
    assert_eq!(word_fuzzy_l_dist("fat furry lazy cat".to_string(), "fluffy furry lazy cat".to_string()), 1);
}
