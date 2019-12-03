use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() -> Result<()> {
    let input = fs::read_to_string("inputs/day3.txt")?;
    let wires = input
        .lines()
        .map(wires_from_str)
        .collect::<Result<Vec<_>, _>>()?;
    let (wire1, wire2) = wires
        .iter()
        .map(Vec::as_slice)
        .map(segments_from_wires)
        .collect_tuple()
        .context("not enough lines")?;
    let intersections = find_intersections(&wire1, &wire2);
    let result = find_nearest_intersection_sum(&intersections).context("no intersection found")?;
    println!("{}", result);
    Ok(())
}

#[allow(dead_code)]
fn find_nearest_intersection(intersections: &[(i64, i64, i64)]) -> Option<i64> {
    intersections
        .iter()
        .map(|(x, y, _)| (x.abs() + y.abs()))
        .min()
}

fn find_nearest_intersection_sum(intersections: &[(i64, i64, i64)]) -> Option<i64> {
    intersections.iter().map(|(_, _, length)| *length).min()
}

fn find_intersections(wire1: &[Segment], wire2: &[Segment]) -> Vec<(i64, i64, i64)> {
    wire1
        .iter()
        .cartesian_product(wire2.iter())
        .filter_map(|(wire1, wire2)| match (wire1, wire2) {
            (
                Segment::Horizontal(x_range, y, horizontal_length, x_start),
                Segment::Vertical(x, y_range, vertical_length, y_start),
            )
            | (
                Segment::Vertical(x, y_range, vertical_length, y_start),
                Segment::Horizontal(x_range, y, horizontal_length, x_start),
            ) if x_range.contains(x) && y_range.contains(y) => Some((
                *x,
                *y,
                horizontal_length + vertical_length + (x - x_start).abs() + (y - y_start).abs(),
            )),
            _ => None,
        })
        .filter(|(x, y, _)| *x != 0 || *y != 0)
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
enum Segment {
    Horizontal(RangeInclusive<i64>, i64, i64, i64), // x_range, y, total_length, x_start
    Vertical(i64, RangeInclusive<i64>, i64, i64),   // x, y_range, total_length, y_start
}

fn segments_from_wires(wires: &[Wire]) -> Vec<Segment> {
    let (mut x, mut y) = (0i64, 0i64);
    let mut total_length = 0;
    wires
        .iter()
        .map(|wire| match wire {
            Wire::Up(length) => {
                let segment = Segment::Vertical(x, y..=y + length, total_length, y);
                y += length;
                total_length += *length;
                segment
            }
            Wire::Down(length) => {
                let segment = Segment::Vertical(x, y - length..=y, total_length, y);
                y -= length;
                total_length += *length;
                segment
            }
            Wire::Left(length) => {
                let segment = Segment::Horizontal(x - length..=x, y, total_length, x);
                x -= length;
                total_length += *length;
                segment
            }
            Wire::Right(length) => {
                let segment = Segment::Horizontal(x..=x + length, y, total_length, x);
                x += length;
                total_length += *length;
                segment
            }
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
enum Wire {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

impl FromStr for Wire {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut chars = s.chars();
        let direction = chars.nth(0).context("invalid wire")?;
        let magnitude = chars.collect::<String>().parse().context("invalid wire")?;
        match direction {
            'U' => Ok(Wire::Up(magnitude)),
            'D' => Ok(Wire::Down(magnitude)),
            'L' => Ok(Wire::Left(magnitude)),
            'R' => Ok(Wire::Right(magnitude)),
            _ => Err(anyhow!("invalid wire")),
        }
    }
}

fn wires_from_str(s: &str) -> Result<Vec<Wire>> {
    s.trim()
        .split(',')
        .map(Wire::from_str)
        .collect::<Result<_, _>>()
}

#[test]
fn test_parsing() {
    assert_eq!(
        wires_from_str("R75,D30,U31,L32").unwrap(),
        vec![
            Wire::Right(75),
            Wire::Down(30),
            Wire::Up(31),
            Wire::Left(32)
        ]
    );
}

#[test]
fn test_part1() {
    assert_eq!(
        find_nearest_intersection(&find_intersections(
            &segments_from_wires(&wires_from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap()),
            &segments_from_wires(&wires_from_str("U62,R66,U55,R34,D71,R55,D58,R83").unwrap()),
        ))
        .unwrap(),
        159
    );
    assert_eq!(
        find_nearest_intersection(&find_intersections(
            &segments_from_wires(
                &wires_from_str("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap()
            ),
            &segments_from_wires(&wires_from_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap()),
        ))
        .unwrap(),
        135
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        find_nearest_intersection_sum(&find_intersections(
            &segments_from_wires(&wires_from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap()),
            &segments_from_wires(&wires_from_str("U62,R66,U55,R34,D71,R55,D58,R83").unwrap()),
        ))
        .unwrap(),
        610
    );
    assert_eq!(
        find_nearest_intersection_sum(&find_intersections(
            &segments_from_wires(
                &wires_from_str("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap()
            ),
            &segments_from_wires(&wires_from_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap()),
        ))
        .unwrap(),
        410
    );
}
