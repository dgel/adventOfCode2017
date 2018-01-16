use std::io::{self, Read};

fn sum_of_adjacent(s: &str, lookahead: usize) -> Option<u32> {
    let mut counter = 0;
    for (c1, c2) in s.chars().zip(s.chars().cycle().skip(lookahead)) {
        match c1.to_digit(10) {
            Some(val) => {
                if c1 == c2 {
                    counter += val;
                }
            }
            None => return None,
        }
    }
    Some(counter)
}

fn main() {
    let mut inp = io::stdin();
    let mut code = String::new();
    if inp.read_to_string(&mut code).is_ok() {
        let code = code.trim();
        match sum_of_adjacent(&code, 1) {
            Some(val) => println!("part 1: {}", val),
            None => println!("Input string invalid: '{}'", code),
        }
        match sum_of_adjacent(&code, code.chars().count() / 2) {
            Some(val) => println!("part 2: {}", val),
            None => println!("Input string invalid: '{}'", code),
        }

    }
}
