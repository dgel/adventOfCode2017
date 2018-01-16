use std::io::{self,Read};

fn run_program<F>(mut ins: Vec<i32>, f: F) -> u32 
    where F: Fn(i32) -> i32 {
    let mut idx: isize = 0;
    let mut ins_count = 0;
    while idx >= 0 && idx < ins.len() as isize {
        let new_idx = idx + ins[idx as usize] as isize;
        ins[idx as usize] = f(ins[idx as usize]);
        idx = new_idx;
        ins_count += 1;
    }
    ins_count
}

fn string_to_nums(s: &str) -> Vec<i32> {
    let mut res = Vec::new();
    for word in s.split_whitespace() {
        if let Ok(num) = word.parse::<i32>() {
            res.push(num);
        }
    }
    res
}

fn main() {
    let mut stdin = io::stdin();

    let mut filecontent = String::new();
    if let Ok(_) = stdin.read_to_string(&mut filecontent) {
        let nums = string_to_nums(&filecontent);
        let num_ins = run_program(nums.clone(), |x| x + 1);
        println!("Program took {} instructions to run", num_ins);
        let num_ins = run_program(nums, |x| if x >= 3 {x - 1} else {x + 1});
        println!("Program took {} instructions to run", num_ins);
    }
}
