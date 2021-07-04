use std::{
    cmp,
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use itertools::Itertools;

use aoc2019::{intcode::*, *};

fn part01(mem: &[i32]) -> Result<i32, Box<dyn Error>> {
    let mut max = 0;
    for settings in (0..=4).permutations(5) {
        let mut output = 0;
        for phase_setting in settings {
            let (mut amplifier, s, r) = IntcodeComputer::with_io(mem.to_owned());

            thread::spawn(move || {
                amplifier.run();
            });

            s.send(phase_setting)?;
            s.send(output)?;

            let (_, out) = r.recv()?;
            // println!("Output ({}): {}", i, out);
            output = out;
        }
        max = cmp::max(max, output);
    }
    Ok(max)
}

fn part02(mem: &[i32]) -> Result<i32, Box<dyn Error>> {
    let mut max = 0;

    for settings in (5..=9).permutations(5) {
        let stop = Arc::new(AtomicBool::new(false));
        let start_amp = |mut amp: IntcodeComputer<i32>, stop: &Arc<AtomicBool>| {
            let thread_stop = stop.clone();
            thread::spawn(move || {
                amp.run();
                thread_stop.store(true, Ordering::SeqCst)
            });
        };

        // Initialize new amps with senders and receivers. Unzip them for easy use.
        let (amps, handlers) = (0..5)
            .map(|_| {
                let (a, s, r) = IntcodeComputer::with_io(mem.to_owned());
                (a, (s, r))
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();
        let (s, r) = handlers.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

        // Start each amp and send its phase setting
        for (i, amp) in amps.into_iter().enumerate() {
            start_amp(amp, &stop);
            s[i].send(settings[i])?;
        }

        // Set initial signal to 0
        let mut output = 0;

        // Keep looping while the amps haven't terminated
        while !stop.load(Ordering::SeqCst) {
            s[0].send(output)?;
            s[1].send(r[0].recv()?.1)?;
            s[2].send(r[1].recv()?.1)?;
            s[3].send(r[2].recv()?.1)?;
            s[4].send(r[3].recv()?.1)?;
            let (_, out) = r[4].recv()?;
            // println!("Output ({}): {}", i, out);
            output = out;
        }

        max = cmp::max(max, output);
    }

    Ok(max)
}

fn day_07() -> (i32, i32) {
    let raw: String = get_input(7).next().unwrap();
    let mem: Vec<i32> = raw.split(',').map(|i| i.parse::<i32>().unwrap()).collect();

    let p1 = part01(&mem).unwrap();
    let p2 = part02(&mem).unwrap();
    (p1, p2)
}

timed_main!(100, day_07());
