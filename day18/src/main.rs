use std::io::{self, Read};
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

extern crate combine;
use combine::*;
use combine::char::{char, digit, letter, spaces, string};

#[derive(Debug, Clone, Copy)]
enum Value {
    Val(i64),
    Ref(char),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(Value),
    Jgz(Value, Value),
}

fn parse(instructions: &str) -> Vec<Instruction> {
    use Instruction::*;
    let num = || {
        (optional(char('-')), many1(digit()).skip(spaces())).map(|(negation, s): (_, String)| {
            let mut n = s.parse::<i64>().unwrap();
            if negation.is_some() {
                n *= -1;
            }
            n
        })
    };
    let ss = |s| string(s).skip(spaces());
    let ch = || letter().skip(spaces());
    let value = || ch().map(Value::Ref).or(num().map(Value::Val));
    let mut snd = (try(ss("snd")), value()).map(|(_, value)| Snd(value));
    let mut set = (ss("set"), ch(), value()).map(|(_, ch, value)| Set(ch, value));
    let mut add = (ss("add"), ch(), value()).map(|(_, ch, value)| Add(ch, value));
    let mut mul = (try(ss("mul")), ch(), value()).map(|(_, ch, value)| Mul(ch, value));
    let mut modp = (ss("mod"), ch(), value()).map(|(_, ch, value)| Mod(ch, value));
    let mut rcv = (ss("rcv"), value()).map(|(_, value)| Rcv(value));
    let mut jgz = (ss("jgz"), value(), value()).map(|(_, value1, value2)| Jgz(value1, value2));
    let ins = choice::<[&mut Parser<Input = State<&str>, Output = Instruction>; 7], _>([&mut snd,
                                                                                        &mut set,
                                                                                        &mut add,
                                                                                        &mut mul,
                                                                                        &mut modp,
                                                                                        &mut rcv,
                                                                                        &mut jgz]);
    let mut inslist = (many1(ins), eof()).map(|(v, _)| v);
    match inslist.parse(State::new(instructions)) {
        Ok((val, _)) => val,
        Err(err) => {
            println!("Error: {}", err);
            vec![]
        }
    }
}

fn get_val(registers: &mut BTreeMap<char, i64>, v: Value) -> i64 {
    match v {
        Value::Val(v) => v,
        Value::Ref(r) => *registers.entry(r).or_insert(0),
    }
}

fn run_instructions<F1, F2, State>(instructions: &[Instruction],
                                   mut registers: BTreeMap<char, i64>,
                                   mut state: State,
                                   snd: F1,
                                   rcv: F2)
                                   -> State
    where F1: Fn(&mut BTreeMap<char, i64>, &mut State, Value),
          F2: Fn(&mut BTreeMap<char, i64>, &mut State, Value) -> bool
{
    use Instruction::*;
    let mut ip = 0i64;

    while ip >= 0 && ip < instructions.len() as i64 {
        let mut skiplen = 1;
        match instructions[ip as usize] {
            Snd(v) => snd(&mut registers, &mut state, v),
            Set(c, v) => {
                let val = get_val(&mut registers, v);
                registers.insert(c, val);
            }
            Add(c, v) => *registers.entry(c).or_insert(0) += get_val(&mut registers, v),
            Mul(c1, v) => *registers.entry(c1).or_insert(0) *= get_val(&mut registers, v),
            Mod(c1, v) => *registers.entry(c1).or_insert(0) %= get_val(&mut registers, v),
            Rcv(v) => {
                if rcv(&mut registers, &mut state, v) {
                    break;
                }
            }
            Jgz(v1, v2) => {
                let conditionval = get_val(&mut registers, v1);
                if conditionval > 0 {
                    skiplen = get_val(&mut registers, v2);
                }
            }
        }
        ip += skiplen;
    }
    state
}

fn part1(instructions: &[Instruction]) -> i64 {
    struct State {
        last_frequency: i64,
        recovered: i64,
    };

    let snd = |registers: &mut BTreeMap<char, i64>, state: &mut State, val| {
        state.last_frequency = get_val(registers, val);
    };
    let rcv = |registers: &mut BTreeMap<char, i64>, state: &mut State, val| {
        let cond = get_val(registers, val) != 0;
        if cond {
            state.recovered = state.last_frequency;
        };
        cond
    };

    run_instructions(instructions,
                     BTreeMap::new(),
                     State {
                         last_frequency: -1,
                         recovered: -1,
                     },
                     snd,
                     rcv)
        .recovered
}

// Part 2

struct DoubleEndedChannelInner {
    queues: [VecDeque<i64>; 2],
    waiting_count: u32,
    closed_end: bool,
}

impl DoubleEndedChannelInner {
    fn new() -> DoubleEndedChannelInner {
        DoubleEndedChannelInner {
            queues: [VecDeque::new(), VecDeque::new()],
            waiting_count: 0,
            closed_end: false,
        }
    }
}

struct DoubleEndedChannel {
    protected: Mutex<DoubleEndedChannelInner>,
    cvar: Condvar,
}

impl DoubleEndedChannel {
    fn new() -> DoubleEndedChannel {
        DoubleEndedChannel {
            protected: Mutex::new(DoubleEndedChannelInner::new()),
            cvar: Condvar::new(),
        }
    }
}

struct ChannelEndPoint {
    data: Arc<DoubleEndedChannel>,
    own_channel: usize,
    other_channel: usize,
}

fn double_ended_channel() -> (ChannelEndPoint, ChannelEndPoint) {
    let channel = Arc::new(DoubleEndedChannel::new());
    (ChannelEndPoint {
         data: Arc::clone(&channel),
         own_channel: 0,
         other_channel: 1,
     },
     ChannelEndPoint {
         data: channel,
         own_channel: 1,
         other_channel: 0,
     })
}


impl ChannelEndPoint {
    fn send(&self, data: i64) {
        let mut guard = self.data.protected.lock().unwrap();
        guard.queues[self.other_channel].push_back(data);
        self.data.cvar.notify_all();
    }

    fn receive(&self) -> Option<i64> {
        let mut guard = self.data.protected.lock().unwrap();
        guard.waiting_count += 1;
        loop {
            match guard.queues[self.own_channel].pop_front() {
                Some(val) => {
                    guard.waiting_count -= 1;
                    return Some(val);
                }
                None => {
                    if guard.closed_end ||
                       (guard.waiting_count == 2 && guard.queues[self.other_channel].is_empty()) {
                        // both threads are waiting and no more data is available
                        self.data.cvar.notify_all();
                        return None;
                    } else {
                        guard = self.data.cvar.wait(guard).unwrap();
                    }
                }
            }
        }
    }
}

impl Drop for ChannelEndPoint {
    fn drop(&mut self) {
        let mut guard = self.data.protected.lock().unwrap();
        guard.closed_end = true;
        self.data.cvar.notify_all();
    }
}

fn part2(instructions: &[Instruction]) -> u64 {
    let (channel0, channel1) = double_ended_channel();

    let snd = || {
        |registers: &mut BTreeMap<char, i64>, state: &mut (ChannelEndPoint, u64), val| {
            state.1 += 1;
            state.0.send(get_val(registers, val));
        }
    };
    let rcv = || {
        |registers: &mut BTreeMap<char, i64>, state: &mut (ChannelEndPoint, u64), val| match val {
            Value::Ref(reg) => {
                match state.0.receive() {
                    Some(val) => {
                        *registers.entry(reg).or_insert(0) = val;
                        false
                    }
                    None => true,
                }
            }
            _ => {
                println!("unexpected value in instruction: {:?}", val);
                true
            }
        }
    };

    let instructions_copy = instructions.to_vec();
    let snd0 = snd();
    let rcv0 = rcv();
    let t0 = thread::spawn(move || {
        run_instructions(&instructions_copy,
                         vec![('p', 0)].into_iter().collect(),
                         (channel0, 0),
                         snd0,
                         rcv0)
    });
    let instructions_copy = instructions.to_vec();
    let snd1 = snd();
    let rcv1 = rcv();
    let t1 = thread::spawn(move || {
        run_instructions(&instructions_copy,
                         vec![('p', 1)].into_iter().collect(),
                         (channel1, 0),
                         snd1,
                         rcv1)
    });
    let _ = t0.join();
    t1.join().unwrap().1
}

fn main() {

    let inp = io::stdin();
    let mut reader = inp.lock();
    let mut input = String::new();
    if reader.read_to_string(&mut input).is_ok() {
        let instructions = parse(&input);
        let part1_result = part1(&instructions);
        println!("part 1: {}", part1_result);
        let part2_result = part2(&instructions);
        println!("part 2: {}", part2_result);
    }
}
