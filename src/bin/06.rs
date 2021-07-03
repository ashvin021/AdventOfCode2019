use std::{collections::HashMap, convert::TryInto};

use aoc2019::*;

fn depth(map: &HashMap<String, String>, start: &str) -> usize {
    let mut parent = start;
    let mut i = 0;
    while let Some(ancestor) = map.get(parent) {
        parent = ancestor;
        i += 1;
    }
    i
}

fn part01(map: &HashMap<String, String>) -> u32 {
    let direct_orbits = map.len();
    let indirect_orbits = map.iter().fold(0, |acc, (_, val)| acc + depth(map, val));

    (direct_orbits + indirect_orbits).try_into().unwrap()
}

fn part02(map: &HashMap<String, String>) -> u32 {
    let mut santas_trace = HashMap::new();
    let mut my_trace = HashMap::new();

    let mut santa_curr = map.get("SAN").unwrap();
    let mut curr = map.get("YOU").unwrap();

    let mut common = None;

    while common.is_none() {
        if let Some(n) = map.get(santa_curr) {
            santas_trace.insert(n.clone(), santa_curr.clone());
            if my_trace.contains_key(n) {
                common = Some(n);
            }
            santa_curr = n;
        }

        if let Some(n) = map.get(curr) {
            my_trace.insert(n.clone(), curr.clone());
            if santas_trace.contains_key(n) {
                common = Some(n);
            }
            curr = n;
        }
    }

    (depth(&santas_trace, common.unwrap()) + depth(&my_trace, common.unwrap()))
        .try_into()
        .unwrap()
}

fn day_06() -> (u32, u32) {
    let map = get_input(6)
        .map(|l| {
            let mut pair = l.split(')').map(str::to_string);
            let value = pair.next().unwrap();
            let key = pair.next().unwrap();
            (key, value)
        })
        .collect::<HashMap<_, _>>();

    let p1 = part01(&map);
    let p2 = part02(&map);
    (p1, p2)
}

timed_main!(100, day_06());
