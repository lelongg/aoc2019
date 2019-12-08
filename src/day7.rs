use anyhow::Result;
use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;
use std::io::Write;

#[allow(dead_code)]
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
    let input = fs::read_to_string("inputs/day7.txt")?;
    let intcode: Vec<i32> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();
    let result = compute_max_thruster_signal_loop(&intcode);
    println!("{:?}", result);
    Ok(())
}

struct Amplifier {
    intcode: Vec<i32>,
    next_inputs: VecDeque<i32>,
    ip: usize,
}

impl Amplifier {
    fn new(intcode: &[i32], phase_setting: i32) -> Self {
        let mut next_inputs = VecDeque::new();
        next_inputs.push_back(phase_setting);
        Self {
            intcode: intcode.to_vec(),
            next_inputs,
            ip: 0,
        }
    }

    fn next_output(&mut self, next_input: i32) -> Option<i32> {
        let Amplifier {
            intcode,
            ip,
            next_inputs,
            ..
        } = self;
        next_inputs.push_back(next_input);
        while let Some(opcode) = intcode.get(*ip).copied().map(Opcode::new) {
            match opcode.code {
                1 => add(&opcode.parameters, intcode, ip),
                2 => mul(&opcode.parameters, intcode, ip),
                3 => input(
                    &mut std::iter::once(next_inputs.pop_front().unwrap()),
                    &opcode.parameters,
                    intcode,
                    ip,
                ),
                4 => {
                    let mut output_data = Vec::new();
                    output(&mut output_data, &opcode.parameters, intcode, ip);
                    return Some(output_data[0]);
                }
                5 => jump_if(&opcode.parameters, intcode, ip),
                6 => jump_unless(&opcode.parameters, intcode, ip),
                7 => is_less_than(&opcode.parameters, intcode, ip),
                8 => is_equal(&opcode.parameters, intcode, ip),
                99 => return None,
                _ => panic!("invalid opcode: `{}`", opcode.code),
            }
        }
        panic!("no end found")
    }
}

fn compute_thruster_signal_loop(intcode: &[i32], phase_settings: &[i32]) -> i32 {
    let mut amplifiers = phase_settings
        .iter()
        .map(|&phase_setting| Amplifier::new(intcode, phase_setting))
        .collect::<Vec<_>>();
    std::iter::successors(Some(0), |&next_input| {
        amplifiers
            .iter_mut()
            .fold(Some(next_input), |next_input, amplifier| {
                next_input.and_then(|next_input| amplifier.next_output(next_input))
            })
    })
    .last()
    .unwrap()
}

fn compute_max_thruster_signal_loop(intcode: &[i32]) -> i32 {
    (5..=9)
        .permutations(5)
        .map(|phase_settings| compute_thruster_signal_loop(&intcode, &phase_settings))
        .max()
        .expect("no max found")
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

#[test]
fn test_compute_thruster_signal() {
    let intcode = &[
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    assert_eq!(
        compute_thruster_signal_loop(intcode, &[9, 8, 7, 6, 5]),
        139_629_729
    );
    assert_eq!(compute_max_thruster_signal_loop(intcode), 139_629_729);
    let intcode = &[
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];
    assert_eq!(
        compute_thruster_signal_loop(intcode, &[9, 7, 8, 5, 6]),
        18216
    );
    assert_eq!(compute_max_thruster_signal_loop(intcode), 18216);
}
