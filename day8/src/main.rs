use std::collections::BTreeMap;
use std::io::{self, Read};
use std::cmp;

extern crate combine;
use combine::*;
use combine::char::{char, string, spaces, letter, digit};

enum Operation {
    Inc(i32),
    Dec(i32),
}

struct Ins {
    register: String,
    operation: Operation,
}


enum Comparison {
    Eq(i32),
    Ne(i32),
    Gt(i32),
    Lt(i32),
    Ge(i32),
    Le(i32),
}

struct Cond {
    register: String,
    comparison: Comparison,
}

#[derive(Debug)]
struct Registers<'a> {
    regs: BTreeMap<&'a str, i32>,
    maxval: Option<i32>,
}

// helper function
fn max(a: Option<i32>, b: i32) -> Option<i32> {
    match a {
        Some(x) => Some(cmp::max(x,b)),
        None => Some(b),
    }
}

impl<'a> Registers<'a> {
    fn new() -> Registers<'a> {
        Registers { regs: BTreeMap::new(), maxval: None}
    }

    fn apply(&mut self, ins: &'a Ins, cond: &'a Cond) {
        let condition_passed = {
            let reg = self.regs.entry(&cond.register).or_insert(0);
            self.maxval = max(self.maxval, *reg);
            match cond.comparison {
                Comparison::Eq(val) => *reg == val,
                Comparison::Ne(val) => *reg != val,
                Comparison::Gt(val) => *reg > val,
                Comparison::Lt(val) => *reg < val,
                Comparison::Ge(val) => *reg >= val,
                Comparison::Le(val) => *reg <= val,
            }
        };
        if condition_passed {
            let reg = self.regs.entry(&ins.register).or_insert(0);
            match ins.operation {
                Operation::Inc(val) => *reg += val,
                Operation::Dec(val) => *reg -= val,
            }
            self.maxval = max(self.maxval, *reg);
        }
    }

    fn largest_value(&self) -> Option<i32> {
        self.regs.iter().max_by_key(|&(_, val)| val).map(|(_, val)| *val)
    }
}

fn parse_script(instructions: &str) -> Option<Vec<(Ins, Cond)>> {
    let ident = || many1(letter());
    let num = || {
        (optional(char('+').or(char('-'))), many1(digit())).map(|(c, s): (_, String)| {
            let mut val = s.parse::<i32>().unwrap();
            if let Some('-') = c {
                val *= -1;
            }
            val
        })
    };

    let inc = (string("inc").skip(spaces()), num()).map(|(_, n)| Operation::Inc(n));
    let dec = (string("dec").skip(spaces()), num()).map(|(_, n)| Operation::Dec(n));
    let opers = inc.or(dec);
    let oper = (ident().skip(spaces()), opers).map(|(id, op)| {
        Ins {
            register: id,
            operation: op,
        }
    });

    let mut eq = (string("==").skip(spaces()), num()).map(|(_, n)| Comparison::Eq(n));
    let mut ne = (string("!=").skip(spaces()), num()).map(|(_, n)| Comparison::Ne(n));
    let mut ge = (try(string(">=")).skip(spaces()), num()).map(|(_, n)| Comparison::Ge(n));
    let mut gt = (string(">").skip(spaces()), num()).map(|(_, n)| Comparison::Gt(n));
    let mut le = (try(string("<=")).skip(spaces()), num()).map(|(_, n)| Comparison::Le(n));
    let mut lt = (string("<").skip(spaces()), num()).map(|(_, n)| Comparison::Lt(n));
    let comps = choice::<[&mut Parser<Input = State<&str>, Output = Comparison>; 6], _>([&mut eq,
                                                                                         &mut ne,
                                                                                         &mut ge,
                                                                                         &mut gt,
                                                                                         &mut le,
                                                                                         &mut lt]);
    let comp = (ident().skip(spaces()), comps).map(|(id, cmp)| {
        Cond {
            register: id,
            comparison: cmp,
        }
    });
    let line = (oper.skip(spaces()), string("if").skip(spaces()), comp.skip(spaces()))
        .map(|(op, _, cmp)| (op, cmp));
    let mut lines = many1(line);

    match lines.parse(State::new(instructions)) {
        Ok((ins, _)) => Some(ins),
        Err(err) => {
            println!("Error: {}", err);
            None
        }
    }
}

fn main() {
    let mut stdin = io::stdin();
    let mut script = String::new();
    if let Ok(_) = stdin.read_to_string(&mut script) {
        if let Some(instructions) = parse_script(&script) {

            let mut registers = Registers::new();
            for &(ref ins, ref cond) in instructions.iter() {
                registers.apply(ins, cond)
            }
            match registers.largest_value() {
                Some(val) => println!("largest value in registers: {}", val),
                None => println!("No values in registers"),
            }
            match registers.maxval {
                Some(val) => println!("largest value over script lifetime: {}", val),
                None => println!("No value use over program lifetime"),
            }
        }

    }
}
