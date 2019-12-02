use anyhow::{anyhow, bail, Result};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day2.txt")?;
    let mut intcode: Vec<usize> = input
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();
    intcode[1] = 12;
    intcode[2] = 2;
    let result = process_intcode(&intcode)?;
    println!("{}", result[0]);
    Ok(())
}

fn process_intcode(intcode: &[usize]) -> Result<Vec<usize>> {
    let mut intcode = intcode.to_vec();
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

#[test]
fn test() {
    assert_eq!(
        *process_intcode(&[1, 0, 0, 0, 99]).unwrap().first().unwrap(),
        2
    );
    assert_eq!(
        *process_intcode(&[2, 3, 0, 3, 99]).unwrap().first().unwrap(),
        2
    );
    assert_eq!(
        *process_intcode(&[2, 4, 4, 5, 99, 0])
            .unwrap()
            .first()
            .unwrap(),
        2
    );
    assert_eq!(
        *process_intcode(&[1, 1, 1, 4, 99, 5, 6, 0, 99])
            .unwrap()
            .first()
            .unwrap(),
        30
    );
}
