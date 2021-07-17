use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use regex::Regex;

use aoc2019::*;

#[derive(Debug)]
struct Chemical {
    name: String,
    qty: u32,
}

fn part01(equations: &HashMap<String, (u32, Vec<Chemical>)>) -> u32 {
    let mut needed = VecDeque::new();
    let mut inventory = HashMap::new();
    let mut num_ores = 0;

    needed.push_back(Chemical {
        name: "FUEL".to_owned(),
        qty: 1,
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

fn deduct_from_inventory(target: &Chemical, inventory: &mut HashMap<String, u32>) -> u32 {
    let mut default = 0;
    let available = inventory.get_mut(&target.name).unwrap_or(&mut default);

    if *available >= target.qty {
        *available -= target.qty;
        0
    } else {
        let required = target.qty - *available;
        *available = 0;
        required
    }
}

fn lookup_reaction(
    target: &String,
    qty: u32,
    equations: &HashMap<String, (u32, Vec<Chemical>)>,
) -> (Vec<Chemical>, u32) {
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

fn day_14() -> (u32, u32) {
    let input_re = Regex::new("(\\d+) (\\w+),? ").unwrap();
    let output_re = Regex::new("=> (\\d+) (\\w+)").unwrap();

    let parse_line = |line: &str| {
        let input = input_re
            .captures_iter(line)
            .map(|cap| Chemical {
                name: cap[2].to_owned(),
                qty: cap[1].parse::<u32>().unwrap(),
            })
            .collect_vec();
        let output = output_re
            .captures(line)
            .map(|cap| (cap[1].parse::<u32>().unwrap(), cap[2].to_owned()))
            .unwrap();

        (output.1, (output.0, input))
    };

    let map: HashMap<_, _> = get_input(14)
        .map(|line| parse_line(&line))
        .into_iter()
        .collect();

    println!("{:?}", map);

    let p1 = part01(&map);
    (p1, 0)
}

timed_main!(1, day_14());
