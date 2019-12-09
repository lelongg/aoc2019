use anyhow::Result;
use std::collections::VecDeque;
use std::fs;
use std::io::Write;

#[allow(dead_code)]
fn read_from_stdin() -> Option<i128> {
    let mut input = String::new();
    print!("input: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    Some(input.trim().parse().expect("not a number"))
}

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day9.txt")?;
    let intcode: Vec<i128> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();
    let mut program = Program::new(&intcode, &[2]);
    let result = std::iter::from_fn(|| program.next_output(&[])).collect::<Vec<_>>();
    println!("{:?}", result);
    Ok(())
}

#[test]
fn test_1() {
    let mut program = Program::new(
        &[
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ],
        &[1],
    );
    let result = std::iter::from_fn(|| program.next_output(&[])).collect::<Vec<_>>();
    assert_eq!(
        &result,
        &[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    );
}

#[test]
fn test_2() {
    let mut program = Program::new(&[1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0], &[]);
    let result = std::iter::from_fn(|| program.next_output(&[])).collect::<Vec<_>>();
    assert_eq!(&result, &[1_219_070_632_396_864]);
}

#[test]
fn test_3() {
    let mut program = Program::new(&[104, 1_125_899_906_842_624, 99], &[]);
    let result = std::iter::from_fn(|| program.next_output(&[])).collect::<Vec<_>>();
    assert_eq!(&result, &[1_125_899_906_842_624]);
}

struct Program {
    intcode: Vec<i128>,
    next_inputs: VecDeque<i128>,
    ip: usize,
    relative_base: i128,
}

impl Program {
    fn new(intcode: &[i128], next_inputs: &[i128]) -> Self {
        Self {
            intcode: intcode.to_vec(),
            next_inputs: next_inputs.iter().copied().collect(),
            ip: 0,
            relative_base: 0,
        }
    }

    fn next_output(&mut self, next_inputs: &[i128]) -> Option<i128> {
        let Self {
            intcode,
            ip,
            relative_base,
            ..
        } = self;
        self.next_inputs
            .append(&mut next_inputs.iter().copied().collect());
        while let Some(opcode) = intcode.get(*ip).copied().map(Opcode::new) {
            match opcode.code {
                1 => add(&opcode.parameters, intcode, ip, relative_base),
                2 => mul(&opcode.parameters, intcode, ip, relative_base),
                3 => input(
                    &mut self.next_inputs.iter().copied(),
                    &opcode.parameters,
                    intcode,
                    ip,
                    relative_base,
                ),
                4 => {
                    let mut output_data = Vec::new();
                    output(
                        &mut output_data,
                        &opcode.parameters,
                        intcode,
                        ip,
                        relative_base,
                    );
                    return Some(output_data[0]);
                }
                5 => jump_if(&opcode.parameters, intcode, ip, relative_base),
                6 => jump_unless(&opcode.parameters, intcode, ip, relative_base),
                7 => is_less_than(&opcode.parameters, intcode, ip, relative_base),
                8 => is_equal(&opcode.parameters, intcode, ip, relative_base),
                9 => shift_relative_base(&opcode.parameters, intcode, ip, relative_base),
                99 => return None,
                _ => panic!("invalid opcode: `{}`", opcode.code),
            }
        }
        panic!("no end found")
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
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
            2 => ParameterMode::Relative,
            _ => panic!("unknown parameter mode"),
        }
    }

    fn get(self, intcode: &mut Vec<i128>, ip: usize, relative_base: i128) -> i128 {
        let index = match self {
            ParameterMode::Position => intcode[ip] as usize,
            ParameterMode::Immediate => ip,
            ParameterMode::Relative => (relative_base as i128 + intcode[ip]) as usize,
        };
        intcode.resize_with(intcode.len().max(index + 1), Default::default);
        intcode[index]
    }

    fn get_mut<'a>(
        &self,
        intcode: &'a mut Vec<i128>,
        ip: usize,
        relative_base: i128,
    ) -> &'a mut i128 {
        let index = match self {
            ParameterMode::Position => intcode[ip] as usize,
            ParameterMode::Immediate => panic!("output cannot be in immediate mode"),
            ParameterMode::Relative => (relative_base as i128 + intcode[ip]) as usize,
        };
        intcode.resize_with(intcode.len().max(index + 1), Default::default);
        &mut intcode[index]
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Parameters(Vec<ParameterMode>);

impl Parameters {
    fn get(&self, index: usize, intcode: &mut Vec<i128>, ip: usize, relative_base: i128) -> i128 {
        self.0
            .get(index)
            .copied()
            .unwrap_or_default()
            .get(intcode, ip + index + 1, relative_base)
    }

    fn get_mut<'a, 'b>(
        &'b self,
        index: usize,
        intcode: &'a mut Vec<i128>,
        ip: usize,
        relative_base: i128,
    ) -> &'a mut i128 {
        self.0.get(index).copied().unwrap_or_default().get_mut(
            intcode,
            ip + index + 1,
            relative_base,
        )
    }
}

struct Opcode {
    code: usize,
    parameters: Parameters,
}

impl Opcode {
    fn new(word: i128) -> Self {
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

fn add(parameters: &Parameters, intcode: &mut Vec<i128>, ip: &mut usize, relative_base: &mut i128) {
    let operand1 = parameters.get(0, intcode, *ip, *relative_base);
    let operand2 = parameters.get(1, intcode, *ip, *relative_base);
    let output = parameters.get_mut(2, intcode, *ip, *relative_base);
    *output = operand1 + operand2;
    *ip += 4;
}

fn mul(parameters: &Parameters, intcode: &mut Vec<i128>, ip: &mut usize, relative_base: &mut i128) {
    let operand1 = parameters.get(0, intcode, *ip, *relative_base);
    let operand2 = parameters.get(1, intcode, *ip, *relative_base);
    let output = parameters.get_mut(2, intcode, *ip, *relative_base);
    *output = operand1 * operand2;
    *ip += 4;
}

fn input(
    stdin: &mut impl Iterator<Item = i128>,
    parameters: &Parameters,
    intcode: &mut Vec<i128>,
    ip: &mut usize,
    relative_base: &mut i128,
) {
    let output = parameters.get_mut(0, intcode, *ip, *relative_base);
    let input = stdin.next().expect("no input");
    *output = input;
    *ip += 2;
}

fn output(
    stdout: &mut Vec<i128>,
    parameters: &Parameters,
    intcode: &mut Vec<i128>,
    ip: &mut usize,
    relative_base: &mut i128,
) {
    let operand1 = parameters.get(0, intcode, *ip, *relative_base);
    stdout.push(operand1);
    *ip += 2;
}

fn jump_if(
    parameters: &Parameters,
    intcode: &mut Vec<i128>,
    ip: &mut usize,
    relative_base: &mut i128,
) {
    let condition = parameters.get(0, intcode, *ip, *relative_base);
    let jump_addr = parameters.get(1, intcode, *ip, *relative_base);
    if condition != 0 {
        *ip = jump_addr as usize;
    } else {
        *ip += 3;
    }
}

fn jump_unless(
    parameters: &Parameters,
    intcode: &mut Vec<i128>,
    ip: &mut usize,
    relative_base: &mut i128,
) {
    let condition = parameters.get(0, intcode, *ip, *relative_base);
    let jump_addr = parameters.get(1, intcode, *ip, *relative_base);
    if condition == 0 {
        *ip = jump_addr as usize;
    } else {
        *ip += 3;
    }
}

fn is_less_than(
    parameters: &Parameters,
    intcode: &mut Vec<i128>,
    ip: &mut usize,
    relative_base: &mut i128,
) {
    let operand1 = parameters.get(0, intcode, *ip, *relative_base);
    let operand2 = parameters.get(1, intcode, *ip, *relative_base);
    let output = parameters.get_mut(2, intcode, *ip, *relative_base);
    *output = if operand1 < operand2 { 1 } else { 0 };
    *ip += 4;
}

fn is_equal(
    parameters: &Parameters,
    intcode: &mut Vec<i128>,
    ip: &mut usize,
    relative_base: &mut i128,
) {
    let operand1 = parameters.get(0, intcode, *ip, *relative_base);
    let operand2 = parameters.get(1, intcode, *ip, *relative_base);
    let output = parameters.get_mut(2, intcode, *ip, *relative_base);
    *output = if operand1 == operand2 { 1 } else { 0 };
    *ip += 4;
}

fn shift_relative_base(
    parameters: &Parameters,
    intcode: &mut Vec<i128>,
    ip: &mut usize,
    relative_base: &mut i128,
) {
    let operand1 = parameters.get(0, intcode, *ip, *relative_base);
    *relative_base = *relative_base as i128 + operand1;
    *ip += 2;
}
