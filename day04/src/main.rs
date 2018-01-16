use std::io::{self, BufRead};
use std::collections::BTreeSet;

fn password_contains_duplicates(password: &str) -> bool {
    let mut wordset = BTreeSet::new();
    for word in password.split_whitespace() {
        if wordset.contains(word) {
            return true;
        }
        wordset.insert(word);
    }
    false
}

fn password_contains_duplicate_anagrams(password: &str) -> bool {
    let mut wordset = BTreeSet::new();
    for word in password.split_whitespace() {
        let mut sorted = Vec::new();
        // just ascii should be fine
        for c in word.bytes() {
            sorted.push(c);
        }
        sorted.sort();
        if wordset.contains(&sorted) {
            return true;
        }
        wordset.insert(sorted);
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let buf_stdin = stdin.lock();

    let mut num_passwords_no_duplicates = 0;
    let mut num_passwords_no_duplicate_anagrams = 0;
    for line in buf_stdin.lines() {
        if let Ok(l) = line {
            if !password_contains_duplicates(&l) {
                num_passwords_no_duplicates += 1;
            }
            if !password_contains_duplicate_anagrams(&l) {
                num_passwords_no_duplicate_anagrams += 1;
            }
        }
    }
    println!("number of passwords with no duplicates: {}", num_passwords_no_duplicates);
    println!("number of passwords with no duplicate anagrams: {}", num_passwords_no_duplicate_anagrams);
}
