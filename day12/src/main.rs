use std::io::{self, BufRead};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn read_lines() -> BTreeMap<i32, Vec<i32>> {
    let mut result = BTreeMap::new();

    let stdin = io::stdin();
    for input in stdin.lock().lines() {
        if let Ok(line) = input {
            let mut parts = line.split(" <-> ");
            if let Some(head) = parts.next() {
                if let Ok(head_number) = head.parse::<i32>() {
                    let mut connections = Vec::new();
                    if let Some(tail) = parts.next() {
                        for connection in tail.split(", ") {
                            if let Ok(connection_number) = connection.parse::<i32>() {
                                connections.push(connection_number);
                            }
                        }
                    }
                    result.insert(head_number, connections);
                }
            }
        }
    }
    result
}

fn group_size(start: i32, connections: &BTreeMap<i32, Vec<i32>>) -> i32 {
    let mut stack = vec![start];
    let mut processed_programs = BTreeSet::new();
    while let Some(num) = stack.pop() {
        if let Some(vec) = connections.get(&num) {
            for connection in vec {
                if !processed_programs.contains(connection) {
                    stack.push(*connection);
                }
            }
        }
        processed_programs.insert(num);
    }
    processed_programs.len() as i32
}

fn count_groups(connections: &mut BTreeMap<i32, Vec<i32>>) -> i32 {
    let mut num_groups = 0;
    while let Some(&key) = connections.keys().next() {
        if let Some(mut stack) = connections.remove(&key) {
            num_groups += 1;
            while let Some(num) = stack.pop() {
                if let Some(vec) = connections.remove(&num) {
                    stack.extend(vec);
                }
            }
        }
    }
    num_groups
}


fn main() {
    let mut connections = read_lines();
    let size = group_size(0, &connections);
    println!("number of programs in group containing 0: {}", size);
    println!("{}", count_groups(&mut connections));
}
