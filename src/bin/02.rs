use itertools::Itertools;

use aoc2019::intcode::*;
use aoc2019::*;

fn part01(computer: &mut IntcodeComputer) -> i32 {
    computer.memory[1] = 12;
    computer.memory[2] = 2;
    computer.run();
    computer.memory[0]
}

fn part02(memory: &Vec<i32>) -> i32 {
    const TARGET: i32 = 19690720;

    for (i, j) in (0..=99).cartesian_product(0..=99) {
        let mut comp = IntcodeComputer::new(memory.clone());
        comp.memory[1] = i;
        comp.memory[2] = j;
        comp.run();

        if TARGET == comp.memory[0] {
            return 100 * i + j;
        }
    }

    -1
}

fn day_02() -> (i32, i32) {
    let raw: String = get_input(2).next().unwrap();
    let memory: Vec<i32> = raw.split(",").map(|i| i.parse::<i32>().unwrap()).collect();
    let mut computer = IntcodeComputer::new(memory.clone());

    let p1 = part01(&mut computer);
    let p2 = part02(&memory);
    (p1, p2)
}

timed_main!(100, day_02());
