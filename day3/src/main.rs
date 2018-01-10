use std::collections::BTreeMap;

enum Dir {
    Right,
    Up,
    Left,
    Down
}

fn add_value(pos: (i32, i32), pos2index: &mut BTreeMap<(i32, i32), usize>, values: &mut Vec<u32>) {
    let mut sum = 0;
    for offset in &[(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)] {
        if let Some(&idx) = pos2index.get(&(pos.0 + offset.0, pos.1 + offset.1)) {
            sum += values[idx];
        }
    }
    let idx = values.len();
    pos2index.insert(pos, idx);
    values.push(sum);
}

fn first_value_larger_spiral(steps: u32) -> u32 {
    let mut location = (0i32, 0i32);

    let mut values = Vec::new();
    values.push(1u32);
    let mut pos2index = BTreeMap::new();
    pos2index.insert(location, 0usize);

    let mut cur_dir = Dir::Right;
    let mut max_abs = 1;
    while *values.last().unwrap() < steps {
        match cur_dir {
            Dir::Right =>
            {
                location = (location.0 + 1, location.1);
                add_value(location, &mut pos2index, &mut values);
                if location.0.abs() == max_abs {
                    cur_dir = Dir::Up;
                }
            },
            Dir::Up =>
            {
                location = (location.0, location.1 + 1);
                add_value(location, &mut pos2index, &mut values);
                if location.1.abs() == max_abs {
                    cur_dir = Dir::Left;
                }
            },
            Dir::Left =>
            {
                location = (location.0 - 1, location.1);
                add_value(location, &mut pos2index, &mut values);
                if location.0.abs() == max_abs {
                    cur_dir = Dir::Down;
                }
            },
            Dir::Down =>
            {
                location = (location.0, location.1 - 1);
                add_value(location, &mut pos2index, &mut values);
                if location.1.abs() == max_abs {
                    cur_dir = Dir::Right;
                    max_abs += 1;
                }
            },
        }
    }
    *values.last().unwrap()
}

fn location_spiral(steps: u32) -> (i32, i32) {
    let mut location = (0i32, 0i32);

    let mut cur_dir = Dir::Right;
    let mut max_abs = 1;
    for _ in 1..steps {
        match cur_dir {
            Dir::Right =>
            {
                location = (location.0 + 1, location.1);
                if location.0.abs() == max_abs {
                    cur_dir = Dir::Up;
                }
            },
            Dir::Up =>
            {
                location = (location.0, location.1 + 1);
                if location.1.abs() == max_abs {
                    cur_dir = Dir::Left;
                }
            },
            Dir::Left =>
            {
                location = (location.0 - 1, location.1);
                if location.0.abs() == max_abs {
                    cur_dir = Dir::Down;
                }
            },
            Dir::Down =>
            {
                location = (location.0, location.1 - 1);
                if location.1.abs() == max_abs {
                    cur_dir = Dir::Right;
                    max_abs += 1;
                }
            },
        }
    }
    location
}

fn main() {
    let (x, y) = location_spiral(265_149);
    println!("part 1: {}", x.abs() + y.abs());
    let val = first_value_larger_spiral(265_149);
    println!("part 2: {}", val);
}

