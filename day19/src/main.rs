use std::io::{self, BufRead};

fn follow_path(lines: &[Vec<char>]) -> (String, u32) {
    let mut result = String::new();
    let mut pos = (0i32, lines[0].iter().position(|&x| x == '|').expect("No starting position found") as i32);
    
    let mut dir = (1i32, 0i32);
    let step = |(x,y), (xdir, ydir)| (x + xdir, y + ydir);
    let get_char = |(x, y): (i32, i32)| if x < 0 || y < 0 || x >= lines.len() as i32 || y >= lines[x as usize].len() as i32 { ' ' } else { lines[x as usize][y as usize] };
    let mut num_steps = 0;

    while get_char(pos) != ' ' {
        match get_char(pos) {
            '|' | '-' => (),
            '+' => {
                let origin = (-dir.0, -dir.1);
                for &new_dir in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    if new_dir != origin && get_char(step(pos, new_dir)) != ' ' {
                        dir = new_dir;
                        break;
                    }
                }
            },
            chr => { result.push(chr); },
        }
        pos = step(pos, dir);
        num_steps += 1;
    }
    (result, num_steps)
}


fn main() {
    let inp = io::stdin();
    let locked = inp.lock();
    let lines = locked.lines().map(|x| x.expect("error reading line").chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let (letters, num_steps) = follow_path(&lines);
    println!("part1: {}", letters);
    println!("part2: {}", num_steps);
}
