use std::io::{self, BufRead};
use std::cmp;

fn evenly_divisible(l: &str) -> Option<i32> {
    let nums = l.split_whitespace().map(|word| word.parse::<i32>()).flat_map(|x| x).collect::<Vec<i32>>();
    for i in 0..nums.len() {
        for j in i+1 .. nums.len() {
            let max = cmp::max(nums[i], nums[j]);
            let min = cmp::min(nums[i], nums[j]);
            if max % min == 0 {
                return Some(max / min)
            }
        }
    }
    None
}

fn linediff(l: &str) -> Option<i32> {
    let mut max = None;
    let mut min = None;
    for word in l.split_whitespace() {
        if let Ok(val) =  word.parse::<i32>() {
            max = Some(max.map_or(val, |x| { cmp::max(val, x) }));
            min = Some(min.map_or(val, |x| { cmp::min(val, x) }));
        }
    }
    match (max, min) {
        (Some(maxval), Some(minval)) => Some(maxval - minval),
        _ => None
    }
}

fn main() {
    let standardin = io::stdin();
    let inp = standardin.lock();
    let mut sum = 0;
    let mut checksum = 0;

    for line in inp.lines() {
        match line {
            Ok(goodline) => { 
                if let Some(val) = evenly_divisible(&goodline) {
                    checksum += val;
                }
                if let Some(val) = linediff(&goodline) {
                    sum += val;
                }
            },
            Err(e) => println!("error reading line: {}", e),
        }
    }

    println!("part 1: {}", sum);
    println!("part 2: {}", checksum);
}
