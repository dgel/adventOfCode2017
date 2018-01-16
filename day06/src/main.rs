use std::io::{self, Read};
use std::collections::BTreeMap;

fn max_idx(arr: &[u8; 16]) -> usize {
    let mut idx = 0;
    let mut max = arr[0];
    for i in 1..16 {
        if arr[i] > max {
            max = arr[i];
            idx = i;
        }
    }
    return idx;
}

fn redistribute(mut banks: [u8; 16]) -> (u32, u32) {
    let mut configurations = BTreeMap::new();
    let mut num_redistributions = 0;
    let mut prev = None;
    while !configurations.contains_key(&banks) {
        configurations.insert(banks, prev);
        prev = Some(banks);
        let mut idx = max_idx(&banks);
        let mut to_distribute = banks[idx];
        banks[idx] = 0;
        while to_distribute > 0 {
            idx = (idx + 1) % 16;
            banks[idx] += 1;
            to_distribute -= 1;
        }
        num_redistributions += 1;
    }
    configurations.insert(banks, prev);
    let mut cycle_size = 1;
    let to_find = banks;
    while prev.unwrap() != to_find {
        cycle_size += 1;
        prev = *configurations.get(&prev.unwrap()).unwrap();
    }
    (num_redistributions, cycle_size)
}

fn read_string(s: &str) -> [u8; 16] {
    let mut idx = 0;
    let mut result = [0; 16];
    for word in s.split_whitespace() {
        if idx >= 16 {
            break;
        }
        if let Ok(num) = word.parse::<u8>() {
            result[idx] = num;
            idx += 1;
        }
    }
    result
}

fn main() {
    let mut stdin = io::stdin();
    let mut inp = String::new();
    if let Ok(_) = stdin.read_to_string(&mut inp) {
        let (num_redists, cycle_size) = redistribute(read_string(&inp));
        println!("number of redistributions: {}\nsize of cycle: {}", num_redists, cycle_size);
    }
}
