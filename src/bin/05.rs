use std::thread;

use aoc2019::{intcode::*, *};

fn part01(mem: &[i32]) -> i32 {
    let (mut computer, s, r) = IntcodeComputer::with_io(mem.to_owned());

    thread::spawn(move || {
        computer.run();
    });

    println!("Input: {}", 1);
    s.send(1).unwrap();

    let mut output = Vec::new();
    for (i, val) in r.iter() {
        println!("Output ({}): {}", i, val);
        output.push((i, val));
    }
    output.pop().unwrap().1
}

fn part02(mem: &[i32]) -> i32 {
    let (mut computer, s, r) = IntcodeComputer::with_io(mem.to_owned());

    thread::spawn(move || {
        computer.run();
    });

    println!("Input: {}", 1);
    s.send(5).unwrap();

    let (i, result) = r.recv().unwrap();
    println!("Output ({}): {}", i, result);
    result
}

fn day_05() -> (i32, i32) {
    let raw: String = get_input(5).next().unwrap();
    let mem: Vec<i32> = raw.split(',').map(|i| i.parse::<i32>().unwrap()).collect();

    let p1 = part01(&mem);
    let p2 = part02(&mem);
    (p1, p2)
}

timed_main!(100, day_05());
