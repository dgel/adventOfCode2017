use std::io;

extern crate combine;
use combine::*;
use combine::char::{char,digit,letter};

#[derive(Debug, Clone, Copy)]
enum Move {
    Spin(u8),
    Exchange(u8, u8),
    Partner(u8, u8)
}

fn parse(input: &str) -> Vec<Move> {
    let num = || many1(digit()).map(|s: String| s.parse::<u8>().unwrap());
    let spin = (char('s'), num()).map(|(_, a)| Move::Spin(a));
    let exchange = (char('x'), num(), char('/'), num()).map(|(_, a, _, b)| Move::Exchange(a, b));
    let partner = (char('p'), letter(), char('/'), letter()).map(|(_, a, _, b)| Move::Partner(a as u8 - 97, b as u8 - 97));
    let mut instructions = (sep_by(spin.or(exchange).or(partner), char(',')), eof()).map(|(v, _)| v);
    
    match instructions.parse(State::new(input)) {
        Ok((val, _)) => val,
        Err(err) => {
            println!("Error: {}", err);
            vec![]
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Dance {
    permutation: [u8; 16],
    mapping: [u8; 16],
}

impl Dance {
    fn double(&mut self) {
        let mut new_permutation = self.permutation;
        for i in 0..16 {
            new_permutation[i] = self.permutation[self.permutation[i] as usize];
        }
        self.permutation = new_permutation;
        let mapping = self.mapping;
        apply_mapping(&mut self.mapping, &mapping);
    }
}

fn apply_permutation(sequence: &mut[u8;16], permutation: &[u8;16]) {
    let mut new_permutation = [0; 16];
    for (i, val) in permutation.iter().enumerate() {
        new_permutation[i] = sequence[*val as usize];
    }
    *sequence = new_permutation;
}

fn apply_mapping(sequence: &mut[u8;16], mapping: &[u8;16]) {
    for i in sequence.iter_mut() {
        *i = mapping[*i as usize];
    }
}

fn apply_n_times(mut dance: Dance, sequence: &mut [u8;16], mut n: u64) {
    while n > 0 {
        if n & 1 > 0 {
            apply_permutation(sequence, &dance.permutation);
            apply_mapping(sequence, &dance.mapping);
        }
        dance.double();
        n >>= 1;
    }
}

fn reduce(instructions: &[Move]) -> Dance {
    let mut dance = Dance{
        permutation: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],
        mapping: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],
    };
    for &ins in instructions {
        match ins {
            Move::Spin(num) => {
                let mut middle = dance.permutation.len() - num as usize;
                let mut first = 0;
                let mut next = middle;
                while first != next {
                    dance.permutation.swap(first, next);
                    first += 1;
                    next += 1;
                    if next == dance.permutation.len() { next = middle; }
                    else if first == middle { middle = next; }
                }
            },
            Move::Exchange(x, y) => dance.permutation.swap(x as usize, y as usize),
            Move::Partner(x, y) => {
                for val in &mut dance.mapping {
                    if *val == x {
                        *val = y;
                    } else if *val == y {
                        *val = x;
                    }
                }
            }
        }
    }
    dance
}


fn print(line: &[u8]) {
    for byte in line {
        print!("{}", (byte + 97) as char);
    }
    println!();
}


fn main() {
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_ok() {
        let instructions = parse(line.trim());
        let dance = reduce(&instructions);
        let mut vals = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        apply_n_times(dance, &mut vals, 1);
        print!("part 1: ");
        print(&vals);
        let mut vals = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        apply_n_times(dance, &mut vals, 1_000_000_000);
        print!("part 2: ");
        print(&vals);
    }
}
