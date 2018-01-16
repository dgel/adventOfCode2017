use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn turn_left(self) -> Self {
        match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn step(&mut self, dir: Dir) {
        match dir {
            Dir::North => self.y += 1,
            Dir::East => self.x += 1,
            Dir::South => self.y -= 1,
            Dir::West => self.x -= 1,
        }
    }
}

#[derive(Clone, Copy)]
enum Status {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Clone)]
struct Map {
    data: Vec<Status>,
    extent: i32, // how far does the map extend from the center
    side: i32,
}

impl Map {
    fn new() -> Self {
        let extent = 25;
        let side = 2 * extent + 1;
        let mut data = Vec::with_capacity(side * side);
        data.resize(side * side, Status::Clean);
        Map {
            data: data,
            extent: extent as i32,
            side: side as i32,
        }
    }

    fn set(&mut self, x: i32, y: i32, value: Status) {
        self.resize(std::cmp::max(x.abs(), y.abs()));
        let x = x + self.extent;
        let y = y + self.extent;
        self.data[(y * self.side + x) as usize] = value;
    }

    fn get(&mut self, x: i32, y: i32) -> &mut Status {
        self.resize(std::cmp::max(x.abs(), y.abs()));
        let x = x + self.extent;
        let y = y + self.extent;
        &mut self.data[(y * self.side + x) as usize]
    }

    fn resize(&mut self, mut new_extent: i32) {
        if new_extent > self.extent {
            new_extent = std::cmp::max(new_extent, self.extent * 2);
            let side = 2 * new_extent + 1;
            let mut data = Vec::with_capacity((side * side) as usize);
            data.resize((side * side) as usize, Status::Clean);
            let mut tmp = Map {
                data: data,
                extent: new_extent,
                side: side,
            };
            for x in -self.extent..self.extent + 1 {
                for y in -self.extent..self.extent + 1 {
                    tmp.set(x, y, *self.get(x, y));
                }
            }
            self.data = tmp.data;
            self.extent = new_extent;
            self.side = side;
        }
    }
}

fn to_map(data: &[String]) -> Map {
    let mut result = Map::new();
    let ymiddle = (data.len() / 2) as i32;
    let xmiddle = (data[0].chars().count() / 2) as i32;
    for (y, line) in data.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                result.set(x as i32 - xmiddle, -(y as i32 - ymiddle), Status::Infected);
            }
        }
    }
    result
}

fn burst(carrier: &mut (Dir, Point), grid: &mut Map, infect_count: &mut usize) {
    let current = grid.get(carrier.1.x, carrier.1.y);
    match *current {
        Status::Infected => {
            *current = Status::Clean;
            carrier.0 = carrier.0.turn_right();
        }
        _ => {
            *current = Status::Infected;
            carrier.0 = carrier.0.turn_left();
            *infect_count += 1;
        }
    }
    carrier.1.step(carrier.0);
}

fn apply_bursts<F>(burst_function: F, mut grid: Map, n: usize) -> usize
where
    F: Fn(&mut (Dir, Point), &mut Map, &mut usize),
{
    let mut carrier = (Dir::North, Point::new(0, 0));
    let mut infect_count = 0;
    for _ in 0..n {
        burst_function(&mut carrier, &mut grid, &mut infect_count);
    }
    infect_count
}

fn burst_evolved(carrier: &mut (Dir, Point), grid: &mut Map, infect_count: &mut usize) {
    let current = grid.get(carrier.1.x, carrier.1.y);
    match *current {
        Status::Weakened => {
            *current = Status::Infected;
            *infect_count += 1;
        }
        Status::Infected => {
            *current = Status::Flagged;
            carrier.0 = carrier.0.turn_right();
        }
        Status::Flagged => {
            *current = Status::Clean;
            carrier.0 = carrier.0.turn_right().turn_right();
        }
        Status::Clean => {
            *current = Status::Weakened;
            carrier.0 = carrier.0.turn_left();
        }
    }
    carrier.1.step(carrier.0);
}

fn main() {
    let inp = io::stdin();
    let buffered = inp.lock();
    let lines = buffered.lines().collect::<Result<Vec<_>, _>>();
    if let Ok(data) = lines {
        let infected_points = to_map(&data);
        let infect_count = apply_bursts(burst, infected_points.clone(), 10_000);
        println!("part 1: {}", infect_count);
        let infect_count = apply_bursts(burst_evolved, infected_points, 10_000_000);
        println!("part 2: {}", infect_count);
    }
}
