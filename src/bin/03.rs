use std::{convert::TryInto, str::FromStr};

use itertools::Itertools;

use aoc2019::*;

#[derive(Debug, Copy, Clone)]
struct WireSection(Direction, i32);

#[derive(Debug, PartialEq, Copy, Clone)]
struct HorizontalSegment {
    x1: i32,
    x2: i32,
    y: i32,
    steps: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct VerticalSegment {
    y1: i32,
    y2: i32,
    x: i32,
    steps: u32,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone)]
struct ParseError;

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match &s[0..1] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => {
                return Err(ParseError);
            }
        };
        Ok(dir)
    }
}

impl FromStr for WireSection {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let magnitude = i32::from_str(&s[1..]).map_err(|_| ParseError)?;
        let dir = Direction::from_str(&s[0..1])?;
        Ok(WireSection(dir, magnitude))
    }
}

fn to_segments(path: &[WireSection]) -> (Vec<HorizontalSegment>, Vec<VerticalSegment>) {
    let (mut x, mut y) = (0, 0);
    let mut steps: u32 = 0;

    let mut horizontal = Vec::new();
    let mut vertical = Vec::new();

    for section in path {
        let WireSection(dir, length) = section;
        let mut d = *length;
        match dir {
            Direction::Left | Direction::Right => {
                if let Direction::Left = dir {
                    d *= -1;
                };
                horizontal.push(HorizontalSegment {
                    x1: x,
                    x2: x + d,
                    y,
                    steps,
                });
                x += d;
            }
            Direction::Up | Direction::Down => {
                if let Direction::Down = dir {
                    d *= -1;
                }
                vertical.push(VerticalSegment {
                    y1: y,
                    y2: y + d,
                    x,
                    steps,
                });
                y += d;
            }
        };
        steps += d.abs() as u32;
    }

    (horizontal, vertical)
}

fn intersect_segments(h: &HorizontalSegment, v: &VerticalSegment) -> Option<i32> {
    let mut xs = [h.x1, h.x2];
    let mut ys = [v.y1, v.y2];
    xs.sort_unstable();
    ys.sort_unstable();

    if (xs[0]..=xs[1]).contains(&v.x) && (ys[0]..=ys[1]).contains(&h.y) {
        Some(v.x + h.y)
    } else {
        None
    }
}

fn intersect_segments_steps(h: &HorizontalSegment, v: &VerticalSegment) -> Option<u32> {
    let mut xs = [h.x1, h.x2];
    let mut ys = [v.y1, v.y2];
    xs.sort_unstable();
    ys.sort_unstable();

    if (xs[0]..=xs[1]).contains(&v.x) && (ys[0]..=ys[1]).contains(&h.y) {
        let h_steps = h.steps + ((v.x - h.x1).abs() as u32);
        let v_steps = v.steps + ((h.y - v.y1).abs() as u32);
        Some(h_steps + v_steps)
    } else {
        None
    }
}

fn part01(segments: &[(Vec<HorizontalSegment>, Vec<VerticalSegment>); 2]) -> i32 {
    let (h1, v1) = &segments[0];
    let (h2, v2) = &segments[1];
    h1.iter()
        .cartesian_product(v2.iter())
        .chain(h2.iter().cartesian_product(v1.iter()))
        .filter_map(|(h, v)| intersect_segments(h, v))
        .filter(|x| *x != 0)
        .min()
        .unwrap()
}

fn part02(segments: &[(Vec<HorizontalSegment>, Vec<VerticalSegment>); 2]) -> u32 {
    let (h1, v1) = &segments[0];
    let (h2, v2) = &segments[1];
    h1.iter()
        .cartesian_product(v2.iter())
        .chain(h2.iter().cartesian_product(v1.iter()))
        .filter_map(|(h, v)| intersect_segments_steps(h, v))
        .filter(|x| *x != 0)
        .min()
        .unwrap()
}

fn day_03() -> (i32, u32) {
    let raw: Vec<String> = get_input(3).collect();
    let segments = raw
        .iter()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<WireSection>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|w| to_segments(&w))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    (part01(&segments), part02(&segments))
}

timed_main!(100, day_03());
