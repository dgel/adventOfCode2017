use std::io::{self, BufRead};
use std::collections::{BTreeMap};

#[derive(Debug, Clone, Copy)]
struct Solution {
    length: u32,
    score: u32,
}

impl Solution {
    fn new() -> Self {
        Solution {
            length: 0,
            score: 0,
        }
    }

    fn add_pair(&mut self, value: (u32, u32)) {
        self.length += 1;
        self.score += value.0;
        self.score += value.1;
    }

    fn pop_pair(&mut self, value: (u32, u32)) {
        self.length -= 1;
        self.score -= value.0;
        self.score -= value.1;
    }
}

fn to_pair(s: &str) -> Option<(u32, u32)> {
    let mut iter = s.split('/');
    iter.next().and_then(|p1| {
        p1.parse::<u32>().ok().and_then(|n1| {
            iter.next()
                .and_then(|p2| p2.parse::<u32>().ok().map(|n2| (n1, n2)))
        })
    })
}

fn get_input() -> Vec<(u32, u32)> {
    let inp = io::stdin();
    let reader = inp.lock();
    let result = reader
        .lines()
        .filter_map(|x| x.ok().and_then(|s: String| to_pair(&s)))
        .collect::<Vec<_>>();
    result
}

struct Node<'a> {
    pair: (u32, u32),
    left_links: &'a Vec<usize>,
    right_links: &'a Vec<usize>,
    used: bool,
}

fn solve_helper(
    partial: &mut Solution,
    best: &mut Solution,
    longest: &mut Solution,
    needed: u32,
    links: &[usize],
    nodes: &mut [Node],
) {
    let mut recurred = false;
    for &val in links.iter() {
        if !nodes[val].used {
            nodes[val].used = true;
            let node = nodes[val].pair;
            partial.add_pair(node);
            let (links, needed) = if node.0 == needed {
                (nodes[val].right_links, node.1)
            } else {
                (nodes[val].left_links, node.0)
            };
            solve_helper(partial, best, longest, needed, links, nodes);
            partial.pop_pair(node);
            nodes[val].used = false;
            recurred = true;
        }
    }

    if !recurred {
        if partial.score > best.score {
            *best = *partial;
        }
        if partial.length > longest.length
            || partial.length == longest.length && partial.score > longest.score
        {
            *longest = *partial;
        }
    }
}

fn solve(pairs: Vec<(u32, u32)>) -> (u32, u32) {
    let mut mapping = BTreeMap::new();
    for (i, &(m, n)) in pairs.iter().enumerate() {
        mapping.entry(m).or_insert_with(|| Vec::new()).push(i);
        mapping.entry(n).or_insert_with(|| Vec::new()).push(i);
    }

    let mut pairs = pairs.into_iter().map(|(x,y)| {
        let left_links = &mapping[&x];
        let right_links = &mapping[&y];
        Node{ pair: (x, y), left_links: left_links, right_links: right_links, used: false }
    }).collect::<Vec<_>>();

    let mut best = Solution::new();
    let mut longest = Solution::new();
    let mut partial = Solution::new();
    for i in 0..pairs.len() {
        let (m, n) = pairs[i].pair;
        if m == 0 || n == 0 {
            pairs[i].used = true;
            partial.add_pair((m, n));
            if m == 0 {
                solve_helper(
                    &mut partial,
                    &mut best,
                    &mut longest,
                    n,
                    pairs[i].right_links,
                    &mut pairs,
                );
            }
            if n == 0 {
                solve_helper(
                    &mut partial,
                    &mut best,
                    &mut longest,
                    m,
                    pairs[i].left_links,
                    &mut pairs,
                );
            }
            partial.pop_pair((m, n));
            pairs[i].used = false;
        }
    }

    (best.score, longest.score)
}

fn main() {
    let (best, longest) = solve(get_input());
    println!("part 1: {}", best);
    println!("part 2: {}", longest);
}
