use anyhow::{anyhow, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day6.txt")?;
    let orbits = parse_orbits(&input)?;
    let result = count_orbits(&orbits);
    println!("{}", result);
    let result = get_minimum_transfer_count(&orbits, "YOU", "SAN");
    println!("{}", result);
    Ok(())
}

fn parse_orbits(input: &str) -> Result<HashMap<String, String>> {
    input
        .lines()
        .map(str::trim)
        .map(
            |line| match line.split(')').collect::<Vec<_>>().as_slice() {
                [parent, child] => Ok(((*child).to_string(), (*parent).to_string())),
                _ => Err(anyhow!("bad orbit")),
            },
        )
        .collect()
}

fn count_orbits(orbits: &HashMap<String, String>) -> u32 {
    let mut orbits_count = 0;
    for (_child, mut parent) in orbits.iter() {
        orbits_count += 1;
        while let Some(grandparent) = orbits.get(parent) {
            parent = grandparent;
            orbits_count += 1;
        }
    }
    orbits_count
}

fn get_parents(orbits: &HashMap<String, String>, child: &str) -> Vec<String> {
    let mut parents = vec![orbits[child].clone()];
    while let Some(grandparent) = parents.last().and_then(|parent| orbits.get(parent)) {
        parents.push(grandparent.to_string());
    }
    parents
}

fn get_minimum_transfer_count(orbits: &HashMap<String, String>, from: &str, to: &str) -> u32 {
    let start_parents = get_parents(orbits, from);
    let destination_parents = get_parents(orbits, to);

    for (i, parent) in start_parents.iter().enumerate() {
        if let Some((j, _)) = destination_parents
            .iter()
            .enumerate()
            .find(|&(_, destination_parent)| destination_parent == parent)
        {
            return i as u32 + j as u32;
        }
    }
    panic!("no path found");
}

#[cfg(test)]
const TEST_INPUT: &str = "\
    COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L";

#[test]
fn test_parse_orbit() {
    let orbits = parse_orbits(TEST_INPUT).unwrap();
    assert_eq!(&orbits["E"], "D");
}

#[test]
fn test_count_orbit() {
    let orbits = parse_orbits(TEST_INPUT).unwrap();
    assert_eq!(count_orbits(&orbits), 42);
}

#[test]
fn test_minimum_transfer() {
    let input = format!(
        "\
        {}
        K)YOU
        I)SAN",
        TEST_INPUT
    );
    let orbits = parse_orbits(&input).unwrap();
    let result = get_minimum_transfer_count(&orbits, "YOU", "SAN");
    assert_eq!(result, 4);
}
