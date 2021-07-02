use aoc2019::{intcode::*, *};

fn day_05() -> (String, String) {
    let raw: String = get_input(5).next().unwrap();
    let mem: Vec<i32> = raw.split(',').map(|i| i.parse::<i32>().unwrap()).collect();
    let mut computer = IntcodeComputer::new(mem);
    computer.run();

    (String::from("N/A"), String::from("N/A"))
}

timed_main!(1, day_05());
