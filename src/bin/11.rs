use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use aoc2019::{intcode::*, *};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Colour {
    Black = 0,
    White = 1,
}

pub struct HullPainter {
    pub painted_panels: HashMap<(i64, i64), Colour>,
    bearing: u32,
    position: (i64, i64),
}

impl HullPainter {
    fn new() -> Self {
        HullPainter {
            painted_panels: HashMap::new(),
            bearing: 0,
            position: (0, 0),
        }
    }

    fn move_next(&mut self, output: i64) {
        assert!(output == 0 || output == 1);
        self.bearing = if output == 0 {
            (self.bearing + 270) % 360
        } else {
            (self.bearing + 90) % 360
        };

        let (x, y) = self.position;
        self.position = match self.bearing {
            0 => (x, y - 1),
            90 => (x + 1, y),
            180 => (x, y + 1),
            270 => (x - 1, y),
            _ => panic!("something went wrong, bearing is an invalid number"),
        };
    }

    fn paint_panel(&mut self, colour: i64) {
        assert!(colour == 0 || colour == 1);

        self.painted_panels.insert(
            self.position,
            if colour == 0 {
                Colour::Black
            } else {
                Colour::White
            },
        );
    }
    fn run(&mut self, mem: &[i64]) {
        let stop = Arc::new(AtomicBool::new(false));

        // Start running the computer
        let (mut computer, s, r) = IntcodeComputer::with_io(mem.to_owned());
        let thread_stop = stop.clone();
        let handle = thread::spawn(move || {
            computer.run();
            thread_stop.store(true, Ordering::SeqCst);
        });

        while !stop.load(Ordering::SeqCst) {
            // Inspect current square
            let curr_colour = self
                .painted_panels
                .get(&self.position)
                .unwrap_or(&Colour::Black);

            // Send the colour to the Intcode computer
            s.send(*curr_colour as i64).unwrap();

            // Paint current square
            if let Ok((_, output)) = r.recv() {
                self.paint_panel(output);
            }

            // Turn and move to next sqare
            if let Ok((_, output)) = r.recv() {
                self.move_next(output);
            }
        }

        handle.join().unwrap();
    }
}

fn part01(mem: &[i64]) -> u32 {
    let mut bot = HullPainter::new();

    // Run the intcode instrutions
    bot.run(mem);

    // Number of panels painted by the bot
    bot.painted_panels.len() as u32
}

fn part02(mem: &[i64]) -> String {
    let mut bot = HullPainter::new();

    // Set the start square to white
    bot.painted_panels.insert((0, 0), Colour::White);

    // Run the intcode instrutions
    bot.run(mem);

    let mut output = [[' '; 45]; 8];

    for ((x, y), colour) in bot.painted_panels {
        output[(y + 1) as usize][(x + 1) as usize] =
            if colour == Colour::White { '#' } else { ' ' };
    }

    for line in output.iter().map(|l| l.iter().collect::<String>()) {
        println!("{}", line);
    }

    "LPZKLGHR".to_owned()
}

fn day_11() -> (u32, String) {
    let raw: String = get_input(11).next().unwrap();
    let mem: Vec<i64> = raw.split(',').map(|i| i.parse::<i64>().unwrap()).collect();

    let p1 = part01(&mem);
    let p2 = part02(&mem);
    (p1, p2)
}

timed_main!(1, day_11());
