use std::cmp;
use std::io;


// representing locations relative to current with points,
// the actual direction to go on the west and east directions
// is different on odd and even x coordinates. this way, going north and then west
// leads to the same hex as going northwest twice
// where the grid is shifted somewhat: 1,0 is northeast of 0,0 and 1,-1 is southeast of the current
// location. In this configuration, going vertically, horizontally and diagonally is all one step.

fn get_distances(input: &str) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    let mut max_dist = 0;

    for direction in input.split(',') {
        match direction {
            "n" => y += 1,
            "s" => y -= 1,
            "ne" => {
                if x & 1 == 0 {
                    x += 1
                } else {
                    x += 1;
                    y += 1
                }
            }
            "nw" => {
                if x & 1 == 0 {
                    x -= 1
                } else {
                    x -= 1;
                    y += 1
                }
            }
            "se" => {
                if x & 1 == 0 {
                    x += 1;
                    y -= 1
                } else {
                    x += 1
                }
            }
            "sw" => {
                if x & 1 == 0 {
                    x -= 1;
                    y -= 1
                } else {
                    x -= 1
                }
            }
            _ => panic!("unexpected token in input"),
        }
        max_dist = cmp::max(max_dist, get_distance(x, y));
    }
    (get_distance(x, y), max_dist)
}

fn get_distance(x: i32, y: i32) -> i32 {
    let abs_x = x.abs();
    let abs_y = if y >= 0 {
        y
    } else {
        if x & 1 == 0 { y.abs() } else { y.abs() - 1 }
    };
    // for every 2 movements in x direction you can move once in y direction
    let diagonals = cmp::min(abs_y, abs_x / 2);
    let x_remainder = abs_x - diagonals;
    let y_remainder = abs_y - diagonals;
    diagonals + x_remainder + y_remainder
}


fn main() {
    let mut line = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut line) {
        let (dist, max_dist) = get_distances(line.trim());
        println!("distance: {}\nmax distance: {}", dist, max_dist);
    }
}
