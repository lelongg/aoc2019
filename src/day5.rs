use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use std::fs;
use std::io::Write;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day5.txt")?;
    let intcode: Vec<i32> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();
    let _result = process_intcode(&intcode, None, None)?;
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl Default for ParameterMode {
    fn default() -> Self {
        ParameterMode::Position
    }
}

impl ParameterMode {
    fn new(mode: u32) -> Self {
        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("unknown parameter mode"),
        }
    }

    fn get(self, intcode: &[i32], ip: usize) -> i32 {
        match self {
            ParameterMode::Position => intcode[intcode[ip] as usize],
            ParameterMode::Immediate => intcode[ip],
        }
    }

    fn get_mut<'a>(&self, intcode: &'a mut [i32], ip: usize) -> &'a mut i32 {
        &mut intcode[intcode[ip] as usize]
    }
}

struct Opcode {
    code: usize,
    parameters_mode: Vec<ParameterMode>,
}

impl Opcode {
    fn new(word: i32) -> Self {
        let mut digits = word.to_string().chars().rev().collect::<Vec<_>>();
        let code = digits
            .drain(0..2.min(digits.len()))
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap();
        let parameters_mode = digits
            .iter()
            .map(|digit| digit.to_digit(10).unwrap())
            .map(ParameterMode::new)
            .collect();
        Opcode {
            code,
            parameters_mode,
        }
    }
}

struct Add;
impl Add {
    fn execute(parameters_mode: &[ParameterMode], intcode: &mut [i32], ip: &mut usize) {
        let operand1 = parameters_mode
            .get(0)
            .copied()
            .unwrap_or_default()
            .get(intcode as &[i32], *ip + 1);
        let operand2 = parameters_mode
            .get(1)
            .copied()
            .unwrap_or_default()
            .get(intcode as &[i32], *ip + 2);
        let output = ParameterMode::Position.get_mut(intcode, *ip + 3);
        *output = operand1 + operand2;
        *ip += 4;
    }
}

struct Mul;
impl Mul {
    fn execute(parameters_mode: &[ParameterMode], intcode: &mut [i32], ip: &mut usize) {
        let operand1 = parameters_mode
            .get(0)
            .copied()
            .unwrap_or_default()
            .get(intcode as &[i32], *ip + 1);
        let operand2 = parameters_mode
            .get(1)
            .copied()
            .unwrap_or_default()
            .get(intcode as &[i32], *ip + 2);
        let output = ParameterMode::Position.get_mut(intcode, *ip + 3);
        *output = operand1 * operand2;
        *ip += 4;
    }
}

struct In;
impl In {
    fn execute(_: &[ParameterMode], intcode: &mut [i32], ip: &mut usize) {
        let output = ParameterMode::Position.get_mut(intcode, *ip + 1);
        let mut input = String::new();
        print!("input: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");
        *output = input.trim().parse().expect("not a number");
        *ip += 2;
    }
}

struct Out;
impl Out {
    fn execute(parameters_mode: &[ParameterMode], intcode: &mut [i32], ip: &mut usize) {
        let operand1 = parameters_mode
            .get(0)
            .copied()
            .unwrap_or_default()
            .get(intcode as &[i32], *ip + 1);
        println!("output: {}", operand1);
        *ip += 2;
    }
}

fn process_intcode(intcode: &[i32], noun: Option<i32>, verb: Option<i32>) -> Result<Vec<i32>> {
    let mut intcode = intcode.to_vec();
    if let Some(noun) = noun {
        intcode[1] = noun;
    }
    if let Some(verb) = verb {
        intcode[2] = verb;
    }
    let mut ip = 0;
    while let Some(opcode) = intcode.get(ip).copied().map(Opcode::new) {
        match opcode.code {
            1 => Add::execute(&opcode.parameters_mode, &mut intcode, &mut ip),
            2 => Mul::execute(&opcode.parameters_mode, &mut intcode, &mut ip),
            3 => In::execute(&opcode.parameters_mode, &mut intcode, &mut ip),
            4 => Out::execute(&opcode.parameters_mode, &mut intcode, &mut ip),
            99 => return Ok(intcode),
            _ => bail!("invalid opcode: `{}`", opcode.code),
        }
    }
    Err(anyhow!("no end found"))
}

#[allow(dead_code)]
fn bruteforce(intcode: &[i32], output: i32) -> Option<(i32, i32)> {
    (0..99).cartesian_product(0..99).find(|&(noun, verb)| {
        process_intcode(intcode, Some(noun), Some(verb)).unwrap()[0] == output
    })
}

#[test]
fn test() {
    assert_eq!(
        *process_intcode(&[1, 0, 0, 0, 99], None, None)
            .unwrap()
            .first()
            .unwrap(),
        2
    );
    assert_eq!(
        *process_intcode(&[2, 3, 0, 3, 99], None, None)
            .unwrap()
            .first()
            .unwrap(),
        2
    );
    assert_eq!(
        *process_intcode(&[2, 4, 4, 5, 99, 0], None, None)
            .unwrap()
            .first()
            .unwrap(),
        2
    );
    assert_eq!(
        *process_intcode(&[1, 1, 1, 4, 99, 5, 6, 0, 99], None, None)
            .unwrap()
            .first()
            .unwrap(),
        30
    );
}
