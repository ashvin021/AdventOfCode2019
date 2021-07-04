use itertools::Itertools;

use aoc2019::intcode::*;
use aoc2019::*;

fn part01(computer: &mut IntcodeComputer<i32>) -> i32 {
    computer.mem[1] = 12;
    computer.mem[2] = 2;
    computer.run();
    computer.mem[0]
}

fn part02(mem: &[i32]) -> i32 {
    const TARGET: i32 = 19690720;

    for (i, j) in (0..=99).cartesian_product(0..=99) {
        let mut comp = IntcodeComputer::new(mem.to_owned());
        comp.mem[1] = i;
        comp.mem[2] = j;
        comp.run();

        if TARGET == comp.mem[0] {
            return 100 * i + j;
        }
    }

    -1
}

fn day_02() -> (i32, i32) {
    let raw: String = get_input(2).next().unwrap();
    let mem: Vec<i32> = raw.split(',').map(|i| i.parse::<i32>().unwrap()).collect();
    let mut computer = IntcodeComputer::new(mem.clone());

    let p1 = part01(&mut computer);
    let p2 = part02(&mem);
    (p1, p2)
}

timed_main!(100, day_02());
