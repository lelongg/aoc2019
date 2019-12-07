use anyhow::{anyhow, bail, Result};
use std::fs;
use std::io::Write;

fn read_from_stdin() -> Option<i32> {
    let mut input = String::new();
    print!("input: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    Some(input.trim().parse().expect("not a number"))
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day5.txt")?;
    let intcode: Vec<i32> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();
    let mut output = Vec::new();
    process_intcode(
        &intcode,
        &mut std::iter::from_fn(read_from_stdin),
        &mut output,
    )?;
    println!("{:?}", output);
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

#[derive(Clone, Debug, PartialEq)]
struct Parameters(Vec<ParameterMode>);

impl Parameters {
    fn get(&self, index: usize, intcode: &[i32], ip: usize) -> i32 {
        self.0
            .get(index)
            .copied()
            .unwrap_or_default()
            .get(intcode, ip + index + 1)
    }

    fn get_mut<'a>(&self, index: usize, intcode: &'a mut [i32], ip: usize) -> &'a mut i32 {
        ParameterMode::Position.get_mut(intcode, ip + index + 1)
    }
}

struct Opcode {
    code: usize,
    parameters: Parameters,
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
        let parameters = Parameters(
            digits
                .iter()
                .map(|digit| digit.to_digit(10).unwrap())
                .map(ParameterMode::new)
                .collect(),
        );
        Opcode { code, parameters }
    }
}

fn add(parameters: &Parameters, intcode: &mut [i32], ip: &mut usize) {
    let operand1 = parameters.get(0, intcode, *ip);
    let operand2 = parameters.get(1, intcode, *ip);
    let output = parameters.get_mut(2, intcode, *ip);
    *output = operand1 + operand2;
    *ip += 4;
}

fn mul(parameters: &Parameters, intcode: &mut [i32], ip: &mut usize) {
    let operand1 = parameters.get(0, intcode, *ip);
    let operand2 = parameters.get(1, intcode, *ip);
    let output = parameters.get_mut(2, intcode, *ip);
    *output = operand1 * operand2;
    *ip += 4;
}

fn input(
    stdin: &mut impl Iterator<Item = i32>,
    parameters: &Parameters,
    intcode: &mut [i32],
    ip: &mut usize,
) {
    let output = parameters.get_mut(0, intcode, *ip);
    let input = stdin.next().expect("no input");
    *output = input;
    *ip += 2;
}

fn output(stdout: &mut Vec<i32>, parameters: &Parameters, intcode: &mut [i32], ip: &mut usize) {
    let operand1 = parameters.get(0, intcode, *ip);
    stdout.push(operand1);
    *ip += 2;
}

fn jump_if(parameters: &Parameters, intcode: &mut [i32], ip: &mut usize) {
    let condition = parameters.get(0, intcode, *ip);
    let jump_addr = parameters.get(1, intcode, *ip);
    if condition != 0 {
        *ip = jump_addr as usize;
    } else {
        *ip += 3;
    }
}

fn jump_unless(parameters: &Parameters, intcode: &mut [i32], ip: &mut usize) {
    let condition = parameters.get(0, intcode, *ip);
    let jump_addr = parameters.get(1, intcode, *ip);
    if condition == 0 {
        *ip = jump_addr as usize;
    } else {
        *ip += 3;
    }
}

fn is_less_than(parameters: &Parameters, intcode: &mut [i32], ip: &mut usize) {
    let operand1 = parameters.get(0, intcode, *ip);
    let operand2 = parameters.get(1, intcode, *ip);
    let output = parameters.get_mut(2, intcode, *ip);
    *output = if operand1 < operand2 { 1 } else { 0 };
    *ip += 4;
}

fn is_equal(parameters: &Parameters, intcode: &mut [i32], ip: &mut usize) {
    let operand1 = parameters.get(0, intcode, *ip);
    let operand2 = parameters.get(1, intcode, *ip);
    let output = parameters.get_mut(2, intcode, *ip);
    *output = if operand1 == operand2 { 1 } else { 0 };
    *ip += 4;
}

fn process_intcode(
    intcode: &[i32],
    stdin: &mut impl Iterator<Item = i32>,
    stdout: &mut Vec<i32>,
) -> Result<()> {
    let mut intcode = intcode.to_vec();
    let mut ip = 0;
    while let Some(opcode) = intcode.get(ip).copied().map(Opcode::new) {
        match opcode.code {
            1 => add(&opcode.parameters, &mut intcode, &mut ip),
            2 => mul(&opcode.parameters, &mut intcode, &mut ip),
            3 => input(stdin, &opcode.parameters, &mut intcode, &mut ip),
            4 => output(stdout, &opcode.parameters, &mut intcode, &mut ip),
            5 => jump_if(&opcode.parameters, &mut intcode, &mut ip),
            6 => jump_unless(&opcode.parameters, &mut intcode, &mut ip),
            7 => is_less_than(&opcode.parameters, &mut intcode, &mut ip),
            8 => is_equal(&opcode.parameters, &mut intcode, &mut ip),
            99 => return Ok(()),
            _ => bail!("invalid opcode: `{}`", opcode.code),
        }
    }
    Err(anyhow!("no end found"))
}

#[test]
fn test_equal_position_mode() {
    let mut output = Vec::new();
    process_intcode(
        &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
        &mut std::iter::once(8),
        &mut output,
    )
    .unwrap();
    assert_eq!(&output, &[1]);

    let mut output = Vec::new();
    process_intcode(
        &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
        &mut std::iter::once(9),
        &mut output,
    )
    .unwrap();
    assert_eq!(&output, &[0]);
}

#[test]
fn test_less_than_position_mode() {
    let mut output = Vec::new();
    process_intcode(
        &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
        &mut std::iter::once(7),
        &mut output,
    )
    .unwrap();
    assert_eq!(&output, &[1]);

    let mut output = Vec::new();
    process_intcode(
        &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
        &mut std::iter::once(9),
        &mut output,
    )
    .unwrap();
    assert_eq!(&output, &[0]);
}

#[test]
fn test_equal_immediate_mode() {
    let mut output = Vec::new();
    process_intcode(
        &[3, 3, 1108, -1, 8, 3, 4, 3, 99],
        &mut std::iter::once(8),
        &mut output,
    )
    .unwrap();
    assert_eq!(&output, &[1]);

    let mut output = Vec::new();
    process_intcode(
        &[3, 3, 1108, -1, 8, 3, 4, 3, 99],
        &mut std::iter::once(9),
        &mut output,
    )
    .unwrap();
    assert_eq!(&output, &[0]);
}

#[test]
fn test_less_than_immediate_mode() {
    let mut output = Vec::new();
    process_intcode(
        &[3, 3, 1107, -1, 8, 3, 4, 3, 99],
        &mut std::iter::once(7),
        &mut output,
    )
    .unwrap();
    assert_eq!(&output, &[1]);

    let mut output = Vec::new();
    process_intcode(
        &[3, 3, 1107, -1, 8, 3, 4, 3, 99],
        &mut std::iter::once(9),
        &mut output,
    )
    .unwrap();
    assert_eq!(&output, &[0]);
}

#[test]
fn test_jump_position_mode() {
    let mut output = Vec::new();
    process_intcode(
        &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        &mut std::iter::once(0),
        &mut output,
    )
    .unwrap();
    assert_eq!(&output, &[0]);
}

#[test]
fn test_jump_immediate_mode() {
    let mut output = Vec::new();
    process_intcode(
        &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        &mut std::iter::once(0),
        &mut output,
    )
    .unwrap();
    assert_eq!(&output, &[0]);
}

#[test]
fn test_jump_and_conditions() {
    let intcode = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    let mut output = Vec::new();
    process_intcode(&intcode, &mut std::iter::once(7), &mut output).unwrap();
    process_intcode(&intcode, &mut std::iter::once(8), &mut output).unwrap();
    process_intcode(&intcode, &mut std::iter::once(9), &mut output).unwrap();
    assert_eq!(&output, &[999, 1000, 1001]);
}
