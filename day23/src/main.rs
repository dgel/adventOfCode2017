use std::io::{self, Read};
use std::collections::BTreeMap;

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
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
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
    let mut set = (ss("set"), ch(), value()).map(|(_, ch, value)| Set(ch, value));
    let mut sub = (try(ss("sub")), ch(), value()).map(|(_, ch, value)| Sub(ch, value));
    let mut mul = (try(ss("mul")), ch(), value()).map(|(_, ch, value)| Mul(ch, value));
    let mut jnz = (ss("jnz"), value(), value()).map(|(_, value1, value2)| Jnz(value1, value2));
    let ins = choice::<[&mut Parser<Input = State<&str>, Output = Instruction>; 4], _>([
        &mut sub, &mut set, &mut mul, &mut jnz
    ]);
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

fn run_instructions(instructions: &[Instruction], mut registers: BTreeMap<char, i64>) -> u64 {
    use Instruction::*;
    let mut ip = 0i64;
    let mut mul_cnt = 0;

    while ip >= 0 && ip < instructions.len() as i64 {
        let mut skiplen = 1;
        match instructions[ip as usize] {
            Set(c, v) => {
                let val = get_val(&mut registers, v);
                registers.insert(c, val);
            }
            Sub(c, v) => *registers.entry(c).or_insert(0) -= get_val(&mut registers, v),
            Mul(c, v) => {
                *registers.entry(c).or_insert(0) *= get_val(&mut registers, v);
                mul_cnt += 1;
            }
            Jnz(v1, v2) => {
                let conditionval = get_val(&mut registers, v1);
                if conditionval != 0 {
                    skiplen = get_val(&mut registers, v2);
                }
            }
        }
        ip += skiplen;
    }
    mul_cnt
}

fn part1(instructions: &[Instruction]) -> u64 {
    run_instructions(instructions, BTreeMap::new())
}

fn translated_assembly() -> usize {
    fn is_prime(b: i64) -> bool {
        let root = ((b as f64).sqrt() + 1.) as i64;
        for d in 2..root {
            if b % d == 0 {
                return false;
            }
        }
        true
    }
    (0..1001i64).filter(|x| !is_prime(x * 17 + 109_900)).count()
}

fn main() {
    let inp = io::stdin();
    let mut reader = inp.lock();
    let mut input = String::new();
    if reader.read_to_string(&mut input).is_ok() {
        let instructions = parse(&input);
        let part1_result = part1(&instructions);
        println!("part 1: {}", part1_result);
        let part2_result = translated_assembly();
        println!("part 2: {}", part2_result);
    }
}
