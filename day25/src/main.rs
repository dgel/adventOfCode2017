use std::io::{self, Read};
use std::collections::BTreeMap;
use std::collections::VecDeque;

extern crate combine;
use combine::*;
use combine::char::{char, digit, letter, string, spaces};

#[derive(Debug)]
struct Tape {
    data: VecDeque<bool>,
    position: usize,
}


#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right
}

impl Tape {
    fn new() -> Self {
        let mut data = VecDeque::with_capacity(200);
        data.push_back(false);
        Tape {
            data: data,
            position: 0,
        }
    }

    fn step(&mut self, dir: Direction) {
        match dir {
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }

    fn left(&mut self) {
        if self.position == 0 {
            self.data.push_front(false);
        } else {
            self.position -= 1;
        }
    }
    fn right(&mut self) {
        self.position += 1;
        if self.position >= self.data.len() {
            self.data.push_back(false);
        }
    }

    fn get(&self) -> bool {
        self.data[self.position]
    }

    fn set(&mut self, val: bool) {
        self.data[self.position] = val;
    }

    fn count_ones(&self) -> u32 {
        self.data.iter().map(|&b| b as u32).sum()
    }

}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    writeval: bool,
    step: Direction,
    new_state: char,
}

#[derive(Debug)]
struct State {
    label: char,
    instructions: [Instruction; 2],
}

#[derive(Debug)]
struct TuringMachine {
    states: BTreeMap<char, State>,
    current_state: char,
    tape: Tape,
}

impl TuringMachine {
    fn new(states: Vec<State>, start_state: char) -> Self {
        TuringMachine {
            states: states.into_iter().map(|s| (s.label, s)).collect(),
            current_state: start_state,
            tape: Tape::new(),
        }
    }

    fn step(&mut self) {
        let val = self.tape.get();
        let instruction = &self.states[&self.current_state].instructions[val as usize];
        self.tape.set(instruction.writeval);
        self.tape.step(instruction.step);
        self.current_state = instruction.new_state;
    }
}

fn parse(input: &str) -> (TuringMachine, u32) {
    let number = || many1(digit()).skip(spaces()).map(|digits: String| digits.parse::<u32>().unwrap());
    let boolval = || char('0').or(char('1')).map(|digit| {
        match digit {
            '0' => false,
            '1' => true,
            _ => panic!("error in parser"),
        }
    });
    let start_state = (string("Begin in state "), letter(), char('.').skip(spaces())).map(|(_, letter, _)| letter);
    let end_after = (string("Perform a diagnostic checksum after "), number(), string("steps.").skip(spaces())).map(|(_, num, _)| num);

    let direction = || string("left").or(string("right")).map(|s| {
        match s {
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => panic!("error in parser"),
        }
    });
    let writeval = || (string("- Write the value "), boolval(), char('.').skip(spaces())).map(|(_, b, _)| b);
    let movement = || (string("- Move one slot to the "), direction(), char('.').skip(spaces())).map(|(_, d, _)| d);
    let next_state = || (string("- Continue with state "), letter(), char('.').skip(spaces())).map(|(_, l, _)| l);
    let instruction = || (writeval(), movement(), next_state()).map(|(w, m, n)| {
        Instruction{ writeval: w, step: m, new_state: n }
    });

    let in_state = || (string("In state "), letter(), char(':').skip(spaces())).map(|(_, l, _)| l);
    let state0 = || (string("If the current value is 0:").skip(spaces()), instruction()).map(|(_, i)| i);
    let state1 = || (string("If the current value is 1:").skip(spaces()), instruction()).map(|(_, i)| i);
    let state = || (in_state(), state0(), state1()).map(|(label, s0, s1)| {
        State {
            label: label,
            instructions: [s0, s1],
        }
    });
    let states = many1(state());

    let mut blueprint = (start_state, end_after, states, eof()).map(|(start, end, states, _)| (start, end, states));

    match blueprint.parse(combine::State::new(input)) {
        Ok((result, _)) => (TuringMachine::new(result.2, result.0), result.1),
        Err(err) => {
            println!("Error parsing: {}", err);
            (TuringMachine::new(Vec::new(), 'X'), 0)
        }
    }

}

fn run_machine(mut machine: TuringMachine, steps: u32) -> u32 {
    for _ in 0..steps {
        machine.step();
    }
    machine.tape.count_ones()
}

fn main() {
    let mut inp = io::stdin();
    let mut input = String::new();
    if inp.read_to_string(&mut input).is_ok() {
        let (machine, steps) = parse(&input);
        let checksum = run_machine(machine, steps);
        println!("part 1: {}", checksum);
    }
}
