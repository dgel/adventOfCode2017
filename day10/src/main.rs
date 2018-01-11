use std::io;
use std::fmt::Write;

fn reverse<T>(list: &mut [T], mut start: usize, mut len: usize) {
    while len > 1 {
        start = start % list.len();
        let endpos = (start + len - 1) % list.len();
        list.swap(start, endpos);
        start = start + 1;
        len -= 2;
    }
}

fn knothash(list: &mut [u32], lengths: &[usize]) {
    let mut skipsize = 0;
    let mut pos = 0;
    for &length in lengths {
        reverse(list, pos, length);
        pos = (pos + length + skipsize) % list.len();
        skipsize += 1;
    }
}

fn knothash2(list: &mut [u32], lengths: &[usize]) {
    let mut skipsize = 0;
    let mut pos = 0;
    for _ in 0..64 {
        for &length in lengths {
            reverse(list, pos, length);
            pos = (pos + length + skipsize) % list.len();
            skipsize += 1;
        }
    }
}

fn to_hex_string(list: &[u32]) -> String {
    let mut result = String::new();
    for value in list.chunks(16).map(|subslice| subslice.iter().fold(0, |a, &b| a ^ b)) {
        write!(&mut result, "{:02x}", value).unwrap();
    }
    result
}

fn parse_input1(line: &str) -> Option<Vec<usize>> {
    let mut res = Vec::new();
    for word in line.split(',').filter(|s| !s.is_empty()) {
        if let Ok(val) = word.trim().parse::<usize>() {
            res.push(val);
        }
        else {
            return None;
        }
    }
    return Some(res);
}

fn parse_input2(line: &str) -> Vec<usize> {
    let mut result = Vec::new();
    for byte in line.bytes() {
        result.push(byte as usize);
    }
    result.extend(&[17, 31, 73, 47, 23]);
    result
}

fn main() {
    let mut line = String::new();
    if let Ok(_) = io::stdin().read_line(&mut line) {
        {
            let lengths = parse_input1(&line).unwrap();
            let mut arr = (0..256).collect::<Vec<_>>();
            knothash(&mut arr, &lengths);
            println!("part 1: {}", arr[0] * arr[1]);
        }
        {
            let lengths = parse_input2(line.trim());
            let mut arr = (0..256).collect::<Vec<_>>();
            knothash2(&mut arr, &lengths);
            println!("part 2: {}", to_hex_string(&arr));

        }
    }
}
