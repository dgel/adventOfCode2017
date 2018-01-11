use std::io::{self, Read};

enum State {
    Default,
    Garbage,
    Cancel,
}

struct Score {
    score: i32,
    num_groups: i32,
    level: i32,
    garbage_chars: i32,
}

fn transition(c: char, s: State, score: &mut Score) -> State {
    match s {
        State::Default => {
            match c {
                '{' => {
                    score.level += 1;
                    State::Default
                },
                '}' => {
                    score.score += score.level;
                    score.num_groups += 1;
                    score.level -= 1;
                    State::Default
                },
                '<' => State::Garbage,
                _ => State::Default,
            }
        },
        State::Garbage => {
            match c {
                '!' => State::Cancel,
                '>' => State::Default,
                _  => {
                    score.garbage_chars += 1;
                    State::Garbage
                }
            }
        },
        State::Cancel => State::Garbage,
    }
}

fn count(input: &str) -> Score {
    let mut score = Score{score: 0, num_groups: 0, level: 0, garbage_chars: 0};
    let mut state = State::Default;
    for c in input.chars() {
        state = transition(c, state, &mut score);
    }
    score
}


fn main() {
    let mut stdin = io::stdin();
    let mut input = String::new();
    if let Ok(_) = stdin.read_to_string(&mut input) {
        let score = count(&input);
        println!("Input had {} groups with a total score of {} and {} pieces of garbage", score.num_groups, score.score, score.garbage_chars);
    }
}
