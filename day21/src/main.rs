use std::fmt;
use std::io::{self, Read};
use std::collections::BTreeMap;

extern crate combine;
use combine::*;
use combine::char::{spaces, string};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Grid {
    data: Vec<bool>,
    len: usize,
}

impl Grid {
    fn default() -> Self {
        Self {
            data: vec![false, true, false, false, false, true, true, true ,true],
            len: 3,
        }
    }

    fn with_len(len: usize) -> Self {
        let mut data = Vec::with_capacity(len * len);
        data.resize(len * len, false);
        Self {data: data, len: len}
    }

    fn from_vec(data: Vec<bool>, len: usize) -> Self {
        if data.len() != len * len {
            panic!("Data length does not correspond to len parameter");
        }
        Self {data: data, len: len}
    }

    fn count_set(&self) -> usize {
        self.data.iter().map(|&val| if val { 1 } else { 0 }).sum()
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        assert!(x < self.len);
        assert!(y < self.len);
        self.data[x * self.len + y] = val;
    }

    fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < self.len);
        assert!(y < self.len);
        self.data[x * self.len + y]
    }

    fn flip(&mut self) {
        for i in 0..self.len {
            let base = i * self.len;
            for j in 0..self.len / 2 {
                self.data.swap(base + j, base + (self.len - j - 1));
            }
        }
    }

    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.data.swap(x1 * self.len + y1, x2 * self.len + y2);
    }

    fn transpose(&mut self) {
        for x in 0 .. self.len {
            for y in x + 1 .. self.len {
                self.swap(x, y, y, x);
            }
        }
    }

    fn rotate(&mut self) {
        self.transpose();
        self.flip();
    }

    fn assign_to_subgrid(&self, subgrid: &mut Grid, x: usize, y: usize) {
        for dx in 0..subgrid.len {
            for dy in 0..subgrid.len {
                subgrid.set(dx, dy, self.get(x + dx, y + dy));
            }
        }
    }

    fn assign_from_subgrid(&mut self, subgrid: &Grid, x: usize, y: usize) {
        for dx in 0..subgrid.len {
            for dy in 0..subgrid.len {
                self.set(x + dx, y + dy, subgrid.get(dx, dy));
            }
        }
    }

}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for chunk in self.data.chunks(self.len) {
            for &val in chunk.iter() {
                if val {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn read_rules(inputfile: &str) -> Vec<(Grid, Grid)> {
    let pattern = || many1(one_of("#./".chars())).map(|pattern: String| {
        let mut data = Vec::new();
        let mut len = 0;
        for (i, c) in pattern.chars().enumerate() {
            match c {
                '#' => data.push(true),
                '.' => data.push(false),
                '/' => if len == 0 { len = i; }
                _ => panic!("something went wrong in combine"),
            }
        }
        Grid::from_vec(data, len)
    });

    let line = (
        pattern().skip(spaces()),
        string("=>").skip(spaces()),
        pattern().skip(spaces()),
    ).map(|(p1, _, p2)| (p1, p2));
    let mut lines = (many1(line), eof()).map(|(rules, _)| rules);

    match lines.parse(State::new(inputfile)) {
        Ok((rules, _)) => rules,
        Err(err) => {
            println!("Error: {}", err);
            Vec::new()
        }
    }
}

fn to_rule_map(rules: Vec<(Grid, Grid)>) -> BTreeMap<Grid, Grid> {
    let mut rule_map = BTreeMap::new();
    for (mut original, transform) in rules {
        for _ in 0..4 {
            rule_map.insert(original.clone(), transform.clone());
            original.rotate();
        }
        original.flip();
        for _ in 0..4 {
            rule_map.insert(original.clone(), transform.clone());
            original.rotate();
        }
    }
    rule_map
}

fn apply_rules(grid: &Grid, rules: &BTreeMap<Grid, Grid>) -> Grid {
    let subgrid_len = grid.len % 2 + 2;
    let num_subgrids = grid.len / subgrid_len;
    let mut tmp_grid = Grid::with_len(subgrid_len);
    let result_subgrid_len = subgrid_len + 1;
    let mut result_grid = Grid::with_len(num_subgrids * (result_subgrid_len));
    for x in 0..num_subgrids {
        for y in 0..num_subgrids {
            grid.assign_to_subgrid(&mut tmp_grid, x * subgrid_len, y * subgrid_len);
            result_grid.assign_from_subgrid(rules.get(&tmp_grid).unwrap(), x * result_subgrid_len, y * result_subgrid_len);
        }
    }
    result_grid
}

fn main() {
    let mut reader = io::stdin();
    let mut input = String::new();
    if reader.read_to_string(&mut input).is_ok() {
        let rules = to_rule_map(read_rules(&input));
        let grids = (0..).scan(Grid::default(), |state, _| {
            *state = apply_rules(state, &rules);
            Some(state.count_set())
        });

        let counts = grids.take(18).collect::<Vec<_>>();
        println!("part 1: {}", counts[4]);
        println!("part 2: {}", counts[17]);
    }
}
