use std::{
    collections::HashMap,
    convert::TryInto,
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use crossbeam::channel::{Receiver, Sender};
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

use aoc2019::{intcode::*, *};

#[derive(Debug, Copy, Clone, Primitive, PartialEq)]
pub enum GameTile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

pub enum ArcadeScreenIn {
    Score(i64),
    Tile(i32, i32, GameTile),
}

pub struct ArcadeCabinet {
    pub screen: HashMap<(i32, i32), GameTile>,
    ball: (i32, i32),
    paddle: (i32, i32),

    // fields for Intcode computer, IO, and termination
    computer: Option<IntcodeComputer<i64>>,
    sender: Sender<i64>,
    receiver: Receiver<(i64, i64)>,
    stopped: Arc<AtomicBool>,
}

impl ArcadeCabinet {
    fn new(mem: &[i64]) -> Self {
        let (computer, sender, receiver) = IntcodeComputer::with_io(mem.to_vec());
        let stopped = Arc::new(AtomicBool::new(false));
        ArcadeCabinet {
            screen: HashMap::new(),
            ball: (0, 0),
            paddle: (0, 0),

            computer: Some(computer),
            sender,
            receiver,
            stopped,
        }
    }

    fn start_computer(&mut self) -> JoinHandle<()> {
        let mut computer = match self.computer.take() {
            Some(c) => c,
            None => panic!("The computer has already been started."),
        };

        let thread_stopped = self.stopped.clone();
        thread::spawn(move || {
            computer.run();
            thread_stopped.store(true, Ordering::SeqCst);
        })
    }

    fn draw_start_screen(&mut self) {
        // Start the Intcode computer
        let handle = self.start_computer();
        let r = &self.receiver;

        // Read tiles while the computer is still running
        while !self.stopped.load(Ordering::SeqCst) {
            if let Ok(ArcadeScreenIn::Tile(x, y, tile)) = Self::receive_screen_in(r) {
                self.screen.insert((x, y), tile);
            }
        }

        handle.join().unwrap();
    }

    fn receive_screen_in(r: &Receiver<(i64, i64)>) -> Result<ArcadeScreenIn, Box<dyn Error>> {
        let x = r.recv()?.1;
        let y = r.recv()?.1;
        let z = r.recv()?.1;

        if (x, y) == (-1, 0) {
            return Ok(ArcadeScreenIn::Score(z));
        }

        let (x, y) = (x.try_into().unwrap(), y.try_into().unwrap());
        let tile = GameTile::from_i64(z).unwrap();
        Ok(ArcadeScreenIn::Tile(x, y, tile))
    }

    fn play_game(&mut self) -> i64 {
        // Insert quarters to play for free
        if let Some(c) = &mut self.computer {
            c.mem[0] = 2;
        }

        // Start the Intcode computer
        let handle = self.start_computer();
        let (s, r) = (&self.sender, &self.receiver);

        let mut score = 0;

        // Read output and react while the computer is still running
        while !self.stopped.load(Ordering::SeqCst) {
            // Read the data and check error condition
            let data = Self::receive_screen_in(r);
            if data.is_err() {
                continue;
            }

            match data.unwrap() {
                ArcadeScreenIn::Tile(x, y, tile) => {
                    // Update paddle
                    if let GameTile::HorizontalPaddle = tile {
                        self.paddle = (x, y);
                    }

                    // Move joystick to follow ball
                    if let GameTile::Ball = tile {
                        self.ball = (x, y);
                        let joystick = self.ball.0.cmp(&self.paddle.0) as i64;
                        s.send(joystick).unwrap();
                    }

                    // Add new tile to the screen
                    self.screen.insert((x, y), tile);
                }
                ArcadeScreenIn::Score(s) => score = s,
            }
        }

        // Join Intcode thread
        handle.join().unwrap();
        score
    }
}

fn part01(mem: &[i64]) -> u32 {
    let mut arcade_cabinet = ArcadeCabinet::new(mem);
    arcade_cabinet.draw_start_screen();
    arcade_cabinet
        .screen
        .values()
        .filter(|v| **v == GameTile::Block)
        .count() as u32
}

fn part02(mem: &[i64]) -> i64 {
    let mut arcade_cabinet = ArcadeCabinet::new(mem);
    arcade_cabinet.play_game()
}

fn day_13() -> (u32, i64) {
    let raw: String = get_input(13).next().unwrap();
    let mem: Vec<i64> = raw.split(',').map(|i| i.parse::<i64>().unwrap()).collect();

    let p1 = part01(&mem);
    let p2 = part02(&mem);
    (p1, p2)
}

timed_main!(1, day_13());
