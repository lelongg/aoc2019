use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day2.txt")?;
    let intcode: Vec<usize> = input
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();
    let result = process_intcode(&intcode, 12, 2)?;
    println!("{}", result[0]);
    let result = bruteforce(&intcode, 19_690_720).unwrap();
    println!("{}", result.0 * 100 + result.1);
    Ok(())
}

fn process_intcode(intcode: &[usize], noun: usize, verb: usize) -> Result<Vec<usize>> {
    let mut intcode = intcode.to_vec();
    intcode[1] = noun;
    intcode[2] = verb;
    let mut ip = 0;
    while let Some(opcode) = intcode.get(ip) {
        match opcode {
            1 => {
                let operand1 = intcode[ip + 1];
                let operand2 = intcode[ip + 2];
                let output = intcode[ip + 3];
                intcode[output] = intcode[operand1] + intcode[operand2];
                ip += 4;
            }
            2 => {
                let operand1 = intcode[ip + 1];
                let operand2 = intcode[ip + 2];
                let output = intcode[ip + 3];
                intcode[output] = intcode[operand1] * intcode[operand2];
                ip += 4;
            }
            99 => return Ok(intcode),
            _ => bail!("invalid opcode: `{}`", opcode),
        }
    }
    Err(anyhow!("no end found"))
}

fn bruteforce(intcode: &[usize], output: usize) -> Option<(usize, usize)> {
    (0..99)
        .cartesian_product(0..99)
        .find(|(noun, verb)| process_intcode(intcode, *noun, *verb).unwrap()[0] == output)
}

#[test]
fn test() {
    assert_eq!(
        *process_intcode(&[1, 0, 0, 0, 99], 0, 0)
            .unwrap()
            .first()
            .unwrap(),
        2
    );
    assert_eq!(
        *process_intcode(&[2, 3, 0, 3, 99], 3, 0)
            .unwrap()
            .first()
            .unwrap(),
        2
    );
    assert_eq!(
        *process_intcode(&[2, 4, 4, 5, 99, 0], 4, 4)
            .unwrap()
            .first()
            .unwrap(),
        2
    );
    assert_eq!(
        *process_intcode(&[1, 1, 1, 4, 99, 5, 6, 0, 99], 1, 1)
            .unwrap()
            .first()
            .unwrap(),
        30
    );
}
