use std::io::{self, Read};
use std::collections::BTreeMap;

extern crate combine;

use combine::*;
use combine::char::{char, digit, spaces, string};

type Vec3 = (i64, i64, i64);

fn abs_sum(p: Vec3) -> i64 {
    p.0.abs() + p.1.abs() + p.2.abs()
}

#[derive(Debug, Clone)]
struct Particle {
    index: usize,
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
}

impl Particle {
    fn step(&mut self) {
        let (x, y, z) = self.position;
        let (xv, yv, zv) = self.velocity;
        let (xa, ya, za) = self.acceleration;
        self.velocity = (xv + xa, yv + ya, zv + za);
        self.position = (x + xv + xa, y + yv + ya, z + zv + za);
    }

    fn distance(&self) -> i64 {
        abs_sum(self.position)
    }

    fn all_signs_match(&self) -> bool {
        let signs_match = |p: i64, v: i64, a: i64| {
            (a == 0 || a.signum() == v.signum()) && ((a == 0 && v == 0) || v.signum() == p.signum())
        };
        let (x, y, z) = self.position;
        let (xv, yv, zv) = self.velocity;
        let (xa, ya, za) = self.acceleration;
        signs_match(x, xv, xa) && signs_match(y, yv, ya) && signs_match(z, zv, za)
    }
}

fn all_signs_match(particles: &[Particle]) -> bool {
    particles.iter().all(|p| p.all_signs_match())
}

fn particle_that_stays_closest(particles: &[Particle]) -> usize {
    let min_acceleration =
        abs_sum(particles.iter().min_by_key(|p| abs_sum(p.acceleration)).unwrap().acceleration);
    let mut lowest_acc = particles.iter()
        .filter(|p| abs_sum(p.acceleration) == min_acceleration)
        .cloned()
        .collect::<Vec<_>>();

    if lowest_acc.len() > 1 {
        while !all_signs_match(&lowest_acc) {
            lowest_acc.iter_mut().for_each(Particle::step);
        }
        let min_velocity =
            abs_sum(lowest_acc.iter().min_by_key(|p| abs_sum(p.velocity)).unwrap().velocity);
        let lowest_velocity = lowest_acc.iter().filter(|p| abs_sum(p.velocity) == min_velocity);
        lowest_velocity.min_by_key(|p| p.distance()).unwrap().index
    } else {
        lowest_acc[0].index
    }
}

fn no_particles_can_collide(particles: &[Particle]) -> bool {
    for i in 0..particles.len() - 1 {
        for j in i + 1..particles.len() {
            let (further, closer) = if particles[i].distance() > particles[j].distance() {
                (&particles[i], &particles[j])
            } else {
                (&particles[j], &particles[i])
            };
            if abs_sum(closer.velocity) > abs_sum(further.velocity) ||
               abs_sum(closer.acceleration) > abs_sum(further.acceleration) {
                return false;
            }

        }
    }

    true
}

fn all_particles_diverging(particles: &mut [Particle]) -> bool {
    let signums = |(x, y, z): (i64, i64, i64)| (x.signum(), y.signum(), z.signum());
    particles.sort_by_key(|p| signums(p.position));
    if !particles.is_empty() {
        let mut slice_start = 0;
        let mut slice_end = 1;
        loop {
            if slice_end == particles.len() {
                if !no_particles_can_collide(&particles[slice_start..slice_end]) {
                    return false;
                }
                break;
            }
            if signums(particles[slice_start].position) != signums(particles[slice_end].position) {
                if !no_particles_can_collide(&particles[slice_start..slice_end]) {
                    return false;
                }
                slice_start = slice_end;
            }
            slice_end += 1;
        }
    }
    true
}

fn number_of_particles_left(mut particles: Vec<Particle>) -> usize {
    while !all_signs_match(&particles) && !all_particles_diverging(particles.as_mut_slice()) {
        particles.iter_mut().for_each(Particle::step);
        let counts = particles.iter().fold(BTreeMap::new(), |mut acc, p| {
            *acc.entry(p.position).or_insert(0) += 1;
            acc
        });
        particles.retain(|p| counts[&p.position] == 1);
    }
    particles.len()
}



fn parse_input(input: &str) -> Vec<(Vec3, Vec3, Vec3)> {
    let number = || {
        (optional(char('-')), many1(digit())).map(|(sign, value): (_, String)| {
            let mut num = value.parse::<i64>().unwrap();
            if sign.is_some() {
                num *= -1;
            }
            num
        })
    };
    let tuple = || {
        (char('<').skip(spaces()),
         number(),
         char(',').skip(spaces()),
         number(),
         char(',').skip(spaces()),
         number(),
         char('>').skip(spaces()))
            .map(|(_, n1, _, n2, _, n3, _)| (n1, n2, n3))
    };
    let pos = (string("p=").skip(spaces()), tuple()).map(|(_, t)| t);
    let vel = (string("v=").skip(spaces()), tuple()).map(|(_, t)| t);
    let acc = (string("a=").skip(spaces()), tuple()).map(|(_, t)| t);
    let line = (pos, char(',').skip(spaces()), vel, char(',').skip(spaces()), acc, spaces())
        .map(|(p, _, v, _, a, _)| (p, v, a));
    let mut lines = many1(line);

    match lines.parse(State::new(input)) {
        Ok((result, _)) => result,
        Err(err) => {
            println!("Error: {}", err);
            vec![]
        }
    }
}


fn main() {
    let mut stdin = io::stdin();
    let mut inp = String::new();
    if stdin.read_to_string(&mut inp).is_ok() {
        let particle_defs = parse_input(&inp);
        let particles = particle_defs.iter()
            .enumerate()
            .map(|(i, &(p, v, a))| {
                Particle {
                    index: i,
                    position: p,
                    velocity: v,
                    acceleration: a,
                }
            })
            .collect::<Vec<_>>();

        let closest = particle_that_stays_closest(&particles);
        println!("part 1: {}", closest);
        let num_left = number_of_particles_left(particles);
        println!("part 2: {}", num_left);
    }

}
