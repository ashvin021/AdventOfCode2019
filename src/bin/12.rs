use std::{collections::HashMap, convert::TryInto};

use gcd::Gcd;
use itertools::{izip, Itertools};

use aoc2019::*;

#[derive(Debug, Clone)]
struct Moon {
    pub pos: [i32; 3],
    pub velocity: [i32; 3],
}

impl Moon {
    fn new(pos: [i32; 3]) -> Self {
        Moon {
            pos,
            velocity: [0; 3],
        }
    }

    fn apply_velocity(&mut self) {
        for (pos, vel) in izip!(&mut self.pos, &self.velocity) {
            *pos += vel;
        }
    }

    fn apply_gravity(&mut self, other: &mut Moon) {
        let compare_axis = |x: i32, other_x: i32| x.cmp(&other_x).reverse() as i32;

        for (vel, other_vel, pos, other_pos) in izip!(
            &mut self.velocity,
            &mut other.velocity,
            &self.pos,
            &other.pos
        ) {
            *vel += compare_axis(*pos, *other_pos);
            *other_vel += compare_axis(*other_pos, *pos);
        }
    }

    fn potential_energy(&self) -> i32 {
        self.pos.iter().map(|x| x.abs()).sum()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.iter().map(|x| x.abs()).sum()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Debug, Clone)]
struct MoonSystem {
    pub moons: Vec<Moon>,
}

impl MoonSystem {
    fn new(start_positions: &[[i32; 3]]) -> Self {
        let moons = start_positions.iter().map(|p| Moon::new(*p)).collect_vec();
        MoonSystem { moons }
    }

    fn total_energy(&self) -> i32 {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }

    fn apply_step(&mut self) {
        for mid in 0..self.moons.len() {
            let (left, right) = self.moons.split_at_mut(mid);
            let right_moon = &mut right[0];
            for left_moon in left {
                right_moon.apply_gravity(left_moon);
            }
        }

        for moon in &mut self.moons {
            moon.apply_velocity();
        }
    }
}

fn lcm(nums: Vec<u32>) -> u64 {
    nums.iter()
        .map(|n| u64::from(*n))
        .fold(1, |acc, x| (acc * x) / acc.gcd(x))
}

fn part01(moons: &mut MoonSystem) -> i32 {
    const STEPS: usize = 1000;
    for _ in 0..STEPS {
        moons.apply_step();
    }
    moons.total_energy()
}

fn part02(moons: &mut MoonSystem) -> u64 {
    let mut periods: [u32; 3] = [0; 3];
    for (i, period) in periods.iter_mut().enumerate() {
        // We use a copy of the system for each iteration
        let mut ms = moons.clone();

        let mut prevcounts = HashMap::new();
        let mut count = 0;
        // The (pos, velocity) i_th-coordinate for each of the moons
        let mut xs: Vec<(i32, i32)> = Vec::new();

        loop {
            if prevcounts.get(&xs).is_some() {
                break;
            }

            prevcounts.insert(xs, count);
            ms.apply_step();

            xs = ms
                .moons
                .iter()
                .map(|m| (m.pos[i], m.velocity[i]))
                .collect_vec();
            count += 1;
        }

        // Get the difference between the current count and previously seen count
        *period = count - prevcounts.get(&xs).unwrap();
    }

    // LCM of the periods for each axis - because they are independent
    lcm(periods.to_vec())
}

fn day_12() -> (i32, u64) {
    let start_positions: Vec<[i32; 3]> = get_input(12)
        .map(|l| {
            let vec = l
                .split('=')
                .filter_map(|val| {
                    let val_str: String =
                        val.chars().take_while(|c| *c != ',' && *c != '>').collect();
                    val_str.parse::<i32>().ok()
                })
                .collect_vec();
            vec.try_into().unwrap()
        })
        .collect_vec();

    let p1 = part01(&mut MoonSystem::new(&start_positions));
    let p2 = part02(&mut MoonSystem::new(&start_positions));
    (p1, p2)
}

timed_main!(1, day_12());
