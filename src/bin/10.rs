#![feature(slice_group_by)]

use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

use aoc2019::*;

fn part01(asteroids: &HashSet<(i8, i8)>) -> (u32, (i8, i8)) {
    let mut max_coords = (0, (0, 0));

    for (i, j) in asteroids {
        let count = asteroids
            .iter()
            // Get relative positions to (i, j)
            .map(|(x, y)| (x - i, y - j))
            // Unique by unit circle angle from (i, j), so direction
            .unique_by(|p| get_angle(p).to_string())
            .count();

        if count > max_coords.0 {
            max_coords = (count, (*i, *j));
        }
    }

    (max_coords.0 as u32, max_coords.1)
}

fn part02(asteroids: &HashSet<(i8, i8)>, station_coords: (i8, i8)) -> i32 {
    const TARGET: usize = 200;

    let (i, j) = station_coords;

    // Get the coords for each asteroid, relative to the station
    let mut relative: Vec<(i8, i8)> = asteroids
        .iter()
        .filter(|p| **p != station_coords)
        .map(|(x, y)| (x - i, y - j))
        .collect();

    // Sort the relative asteroids by angle first and distance second
    relative.sort_unstable_by(|a, b| {
        (get_angle(a))
            .partial_cmp(&get_angle(b))
            .unwrap_or(Ordering::Equal)
            .reverse()
            .then(a.1.abs().cmp(&b.1.abs()))
            .then(a.0.abs().cmp(&b.0.abs()))
    });

    // Group the asteroids by angle
    let mut groups = relative
        .group_by(|a, b| (get_angle(a) - get_angle(b)).abs() < f64::EPSILON)
        .map(|ps| ps.iter())
        .collect_vec();

    let mut count = 0;
    let mut target = (0, 0);
    'outer: loop {
        // One clockwise rotation
        for group in &mut groups {
            // Take the next element from the i_th group
            if let Some(coord) = group.next() {
                target = *coord;
                count += 1;
            }

            // Break on the TARGET element
            if count == TARGET {
                break 'outer;
            }
        }
    }

    let (x, y) = target;
    i32::from(x + i) * 100 + i32::from(y + j)
}

fn get_angle((x, y): &(i8, i8)) -> f64 {
    (f64::from(*x)).atan2(f64::from(*y))
}

fn day_10() -> (u32, i32) {
    let asteroids: HashSet<(i8, i8)> = get_input(10)
        .enumerate()
        .map(|(j, s)| {
            s.chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if c == '#' {
                        Some((i as i8, j as i8))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let (count, max_coord) = part01(&asteroids);
    let p2 = part02(&asteroids, max_coord);
    (count, p2)
}

timed_main!(100, day_10());
