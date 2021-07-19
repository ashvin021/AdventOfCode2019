use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

use itertools::Itertools;
use regex::Regex;

use aoc2019::*;

#[derive(Debug)]
struct Chemical {
    name: String,
    qty: u64,
}

fn ores_needed(equations: &HashMap<String, (u64, Vec<Chemical>)>, qty: u64) -> u64 {
    let mut needed = VecDeque::new();
    let mut inventory = HashMap::new();
    let mut num_ores = 0;

    needed.push_back(Chemical {
        name: "FUEL".to_owned(),
        qty,
    });

    loop {
        let target = match needed.pop_front() {
            Some(chemical) => chemical,
            None => break,
        };

        if target.name == "ORE" {
            num_ores += target.qty;
            continue;
        }

        let required = deduct_from_inventory(&target, &mut inventory);
        let (ingredients, leftovers) = lookup_reaction(&target.name, required, equations);

        needed.extend(ingredients.into_iter());
        *inventory.entry(target.name).or_insert(0) += leftovers;
    }

    num_ores
}

fn deduct_from_inventory(target: &Chemical, inventory: &mut HashMap<String, u64>) -> u64 {
    let mut default = 0;
    let available = inventory.get_mut(&target.name).unwrap_or(&mut default);

    let required = target.qty.checked_sub(*available).unwrap_or(0);
    *available = (*available).checked_sub(target.qty).unwrap_or(0);
    required
}

fn lookup_reaction(
    target: &String,
    qty: u64,
    equations: &HashMap<String, (u64, Vec<Chemical>)>,
) -> (Vec<Chemical>, u64) {
    let (output_qty, ingredients) = equations.get(target).unwrap();
    let factor = (qty + (output_qty - 1)) / output_qty;

    let scaled = ingredients
        .iter()
        .map(|c| Chemical {
            name: c.name.clone(),
            qty: c.qty * factor,
        })
        .collect_vec();
    (scaled, (output_qty * factor) - qty)
}

fn part01(equations: &HashMap<String, (u64, Vec<Chemical>)>) -> u64 {
    ores_needed(equations, 1)
}

fn part02(equations: &HashMap<String, (u64, Vec<Chemical>)>, available_ore: u64) -> u64 {
    let mut lo = available_ore / ores_needed(equations, 1);
    let mut hi = available_ore;
    let mut mid = (lo + hi) / 2;

    while lo < hi - 1 {
        mid = (lo + hi) / 2;
        match ores_needed(equations, mid).cmp(&available_ore) {
            Ordering::Less => lo = mid,
            Ordering::Greater => hi = mid,
            Ordering::Equal => break,
        }
    }

    mid
}

fn day_14() -> (u64, u64) {
    let input_re = Regex::new("(\\d+) (\\w+),? ").unwrap();
    let output_re = Regex::new("=> (\\d+) (\\w+)").unwrap();

    let parse_line = |line: &str| {
        let input = input_re
            .captures_iter(line)
            .map(|cap| Chemical {
                name: cap[2].to_owned(),
                qty: cap[1].parse::<u64>().unwrap(),
            })
            .collect_vec();
        let output = output_re
            .captures(line)
            .map(|cap| (cap[1].parse::<u64>().unwrap(), cap[2].to_owned()))
            .unwrap();

        (output.1, (output.0, input))
    };

    let map: HashMap<_, _> = get_input(14)
        .map(|line| parse_line(&line))
        .into_iter()
        .collect();

    let p1 = part01(&map);

    const AVAILABLE_ORE: u64 = 1_000_000_000_000;
    let p2 = part02(&map, AVAILABLE_ORE);
    (p1, p2)
}

timed_main!(1, day_14());
