use std::thread;

use aoc2019::{intcode::*, *};

fn part01(mem: &[i64]) -> i64 {
    let (mut computer, s, r) = IntcodeComputer::with_io(mem.to_owned());

    thread::spawn(move || {
        computer.run();
    });

    let input = 1;
    println!("Input: {}", input);
    s.send(input).unwrap();

    let mut output = Vec::new();
    for (i, val) in r.iter() {
        println!("Output ({}): {}", i, val);
        output.push((i, val));
    }
    output.pop().unwrap().1
}

fn part02(mem: &[i64]) -> i64 {
    let (mut computer, s, r) = IntcodeComputer::with_io(mem.to_owned());

    thread::spawn(move || {
        computer.run();
    });

    let input = 2;
    println!("Input: {}", input);
    s.send(input).unwrap();

    let (i, result) = r.recv().unwrap();
    println!("Output ({}): {}", i, result);
    result
}

fn day_09() -> (i64, i64) {
    let raw: String = get_input(9).next().unwrap();
    let mem: Vec<i64> = raw.split(',').map(|i| i.parse::<i64>().unwrap()).collect();

    let p1 = part01(&mem);
    let p2 = part02(&mem);
    (p1, p2)
}

timed_main!(1, day_09());
