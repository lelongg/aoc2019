use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;

fn main() -> Result<()> {
    let mut file = BufReader::new(File::open("inputs/day1.txt")?);
    part1(&mut file);
    file.seek(SeekFrom::Start(0))?;
    part2(&mut file);
    Ok(())
}

fn part1(file: &mut impl BufRead) {
    let result: u32 = file
        .lines()
        .map(|mass| mass.unwrap().parse().unwrap())
        .map(compute_module_fuel)
        .sum();
    println!("{}", result);
}

fn part2(file: &mut impl BufRead) {
    let result: u32 = file
        .lines()
        .map(|mass| mass.unwrap().parse().unwrap())
        .map(compute_module_fuel_2)
        .sum();
    println!("{}", result);
}

fn compute_module_fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn compute_module_fuel_2(mass: u32) -> u32 {
    let mut sum = 0;
    let mut mass = mass;
    while mass != 0 {
        mass = compute_module_fuel(mass);
        sum += mass;
    }
    sum
}

#[test]
fn test_part1() {
    assert_eq!(compute_module_fuel(12), 2);
    assert_eq!(compute_module_fuel(14), 2);
    assert_eq!(compute_module_fuel(1969), 654);
    assert_eq!(compute_module_fuel(100_756), 33_583);
}

#[test]
fn test_part2() {
    assert_eq!(compute_module_fuel_2(14), 2);
    assert_eq!(compute_module_fuel_2(1969), 966);
    assert_eq!(compute_module_fuel_2(100_756), 50_346);
}
