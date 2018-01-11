use std::io::{self, BufRead};
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct Layer {
    range: i32,
    position: i32,
    direction: i32,
}

fn read_input() -> Vec<Layer> {
    let stdin = io::stdin();
    let mut result = Vec::new();
    for input in stdin.lock().lines() {
        if let Ok(line) = input {
            let mut parts = line.split(": ");
            if let Some(depth) = parts.next() {
                if let Some(range) = parts.next() {
                    if let Ok(depth_num) = depth.parse::<usize>() {
                        if let Ok(range_num) = range.parse::<i32>() {
                            while result.len() <= depth_num {
                                result.push(Layer{range: 0, position: 0, direction: 1});
                            }
                            result[depth_num].range = range_num;
                        }
                    }
                }
            }
        }
    }
    result
}

fn step_layers(layers: &mut Vec<Layer>) {
    for layer in layers.iter_mut() {
        if layer.range > 0 {
            layer.position += layer.direction;
            if layer.position == 0 || layer.position == layer.range - 1 {
                layer.direction *= -1;
            }
        }
    }
}

fn walk_severity(mut layers: Vec<Layer>) -> i32 {
    let mut severity = 0;

    for packet_position in 0..layers.len() {
        if layers[packet_position].range > 0 && layers[packet_position].position == 0 {
            severity += packet_position as i32 * layers[packet_position].range;
        }
        step_layers(&mut layers);
    }
    severity
}

fn is_caught(mut layers: Vec<Layer>) -> bool {
    for packet_position in 0..layers.len() {
        if layers[packet_position].range > 0 && layers[packet_position].position == 0 {
            return true;
        }
        step_layers(&mut layers);
    }
    false
}

fn layer_period(layers: &Vec<Layer>) -> i32 {
    let mut layer_ranges = BTreeSet::new();
    for layer in layers.iter() {
        if layer.range > 0 {
            // the period of one range is 2*(range - 1)
            // i.e. the number of steps to return to the starting position
            layer_ranges.insert(2 * (layer.range - 1));
        }
    }
    fn gcd(mut a: i32, b: &i32) -> i32 {
        let mut b = *b;
        while b != 0 {
            let tmp = a % b;
            a = b;
            b = tmp;
        }
        a
    }
    let lcm = |a, b| {
        let res = (a * b) / gcd(a, b);
        res
    };
    // get lowest common multiple of all
    layer_ranges.iter().fold(1, lcm)
}

fn minimum_wait(mut layers: Vec<Layer>) -> Option<i32> {
    // after the period, the layers should all be back to their starting position
    let period = layer_period(&layers);
    for i in 0..period {
        let caught = is_caught(layers.clone());
        if !caught {
            return Some(i);
        }
        step_layers(&mut layers);
    }
    None
}


fn main() {
    let layers = read_input();
    let severity = walk_severity(layers.clone());
    println!("severity when starting at picosecond 0: {}", severity);
    if let Some(steps) = minimum_wait(layers) {
        println!("Not caught after waiting {} steps", steps);
    } else {
        println!("No way to not get caught");
    }
}
