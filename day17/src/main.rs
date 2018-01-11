use std::collections::VecDeque;


fn short_circuit(n: usize) -> u32 {
    let mut buffer = VecDeque::new();
    buffer.push_back(0);
    for i in 1..2018 {
        for _ in 0..n {
            let x = buffer.pop_back().unwrap();
            buffer.push_front(x);
        }
        buffer.push_front(i);
    }
    *buffer.back().unwrap()
}

fn short_circuit2(n: usize) -> u32 {
    let mut buffer = VecDeque::new();
    buffer.push_back(0);
    for i in 1..50_000_001 {
        for _ in 0..n {
            let x = buffer.pop_back().unwrap();
            buffer.push_front(x);
        }
        buffer.push_front(i);
    }
    while *buffer.front().unwrap() != 0 {
        let x = buffer.pop_back().unwrap();
        buffer.push_front(x);
    }
    *buffer.back().unwrap()
}

fn short_circuit2_alt(n: usize) -> u32 {
    let mut pos = 0;
    let mut val_at_pos_1 = 0;
    let mut len = 1;
    for i in 1..50_000_001 {
        pos = (pos + n) % len + 1;
        len += 1;
        if pos == 1 {
            val_at_pos_1 = i;
        }
    }
    val_at_pos_1
}


fn main() {
    println!("part 1: {}", short_circuit(394));
    println!("part 2: {}", short_circuit2(394));
    println!("part 2 (alt): {}", short_circuit2_alt(394));
}
