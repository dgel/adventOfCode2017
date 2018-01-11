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
    for _ in 0..64 {
        for &length in lengths {
            reverse(list, pos, length);
            pos = (pos + length + skipsize) % list.len();
            skipsize += 1;
        }
    }
}

fn hash(line: String) -> Vec<u32> {
    let mut result = Vec::new();
    for byte in line.bytes() {
        result.push(byte as usize);
    }
    result.extend(&[17, 31, 73, 47, 23]);
    let mut arr = (0..256).collect::<Vec<_>>();
    knothash(&mut arr, &result);
    arr.chunks(16).map(|subslice| subslice.iter().fold(0, |a, &b| a ^ b)).collect::<Vec<_>>()
}

fn hash_to_bitvec(vals: &Vec<u32>) -> Vec<bool> {
    let mut res = Vec::new();
    for val in vals {
        for i in (0..8).rev() {
            res.push((val & (1 << i)) > 0);
        }
    }
    res
}

fn mark_region(vals: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    let mut stack = vec![(i, j)];
    while let Some((i, j)) = stack.pop() {
        if vals.len() > i + 1 && vals[i + 1][j] {
            stack.push((i + 1, j));
        }
        if i > 0 && vals[i - 1][j] {
            stack.push((i - 1, j));
        }
        if vals[i].len() > j + 1 && vals[i][j + 1] {
            stack.push((i, j + 1));
        }
        if j > 0 && vals[i][j - 1] {
            stack.push((i, j - 1));
        }
        vals[i][j] = false;
    }
}

fn count_regions(mut vals: Vec<Vec<bool>>) -> u32 {
    let mut res = 0;
    let length = vals.len();
    for i in 0..length {
        let width = vals[i].len();
        for j in 0..width {
            if vals[i][j] {
                mark_region(&mut vals, i, j);
                res += 1;
            }
        }
    }
    res
}

fn main() {
    let input = "oundnydw";
    let mut total_bits_set = 0;
    let mut values = Vec::new();
    for i in 0..128 {
        let line = format!("{}-{}", input, i);
        let hashvalue = hash(line);
        for val in hashvalue.iter() {
            total_bits_set += val.count_ones();
        }
        values.push(hash_to_bitvec(&hashvalue));
    }
    println!("total bits used: {}", total_bits_set);
    println!("total number of regions: {}", count_regions(values));
}
