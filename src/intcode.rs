use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
    io,
};

use crossbeam::channel::{unbounded, Receiver, Sender};
use num_traits::{cast::cast, FromPrimitive, Num, NumCast, One, Zero};

#[derive(Debug)]
pub struct IntcodeComputer<T: Num> {
    pub mem: Vec<T>,
    instr_ptr: usize,
    relative_base: isize,
    incoming: Option<Receiver<T>>,
    outgoing: Option<Sender<(T, T)>>,
}

#[derive(Primitive)]
pub enum IntcodeOpcode {
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    JumpEq = 5,
    JumpNeq = 6,
    LessThan = 7,
    Equals = 8,
    RelBase = 9,
    Halt = 99,
}

#[derive(Primitive)]
enum ParamMode {
    PositionMode = 0,
    ImmediateMode = 1,
    RelativeMode = 2,
}

impl<T> IntcodeComputer<T>
where
    T: Num + Copy + Clone + PartialOrd + NumCast + Display + From<bool>,
    u8: Into<T>,
{
    pub const MEM_SIZE: usize = 65535;

    pub fn new(mut mem: Vec<T>) -> Self {
        mem.resize(Self::MEM_SIZE, Zero::zero());
        IntcodeComputer {
            mem,
            instr_ptr: 0,
            relative_base: 0,
            incoming: None,
            outgoing: None,
        }
    }

    pub fn with_io(mut mem: Vec<T>) -> (Self, Sender<T>, Receiver<(T, T)>) {
        let (s_input, r_input) = unbounded();
        let (s_output, r_output) = unbounded();
        mem.resize(Self::MEM_SIZE, Zero::zero());

        (
            IntcodeComputer {
                mem,
                instr_ptr: 0,
                relative_base: 0,
                incoming: Some(r_input),
                outgoing: Some(s_output),
            },
            s_input,
            r_output,
        )
    }

    pub fn run(&mut self) {
        while self.execute_instruction().unwrap() {}
    }

    fn execute_instruction(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let i = self.instr_ptr;

        // Get opcode and param indexes
        let opcode = IntcodeOpcode::from_u8(cast(self.mem[i] % 100.into()).unwrap()).unwrap();
        let modes = Self::get_modes(self.mem[i] / 100.into(), &opcode);
        let indices: Vec<usize> = (i + 1..)
            .zip(modes.iter())
            .map(|(index, mode)| Self::fetch_param_index(&self, index, mode))
            .collect();

        // Perform operation
        match opcode {
            IntcodeOpcode::Add => {
                self.mem[indices[2]] = self.mem[indices[0]] + self.mem[indices[1]];
            }
            IntcodeOpcode::Mult => {
                self.mem[indices[2]] = self.mem[indices[0]] * self.mem[indices[1]];
            }
            IntcodeOpcode::Input => {
                self.mem[indices[0]] = self.receive_input();
            }
            IntcodeOpcode::Output => self.send_output(cast(i).unwrap(), self.mem[indices[0]]),
            IntcodeOpcode::JumpEq => {
                if self.mem[indices[0]] != 0.into() {
                    self.instr_ptr = cast(self.mem[indices[1]]).ok_or("couldn't cast to usize")?;
                    return Ok(true);
                }
            }
            IntcodeOpcode::JumpNeq => {
                if self.mem[indices[0]] == 0.into() {
                    self.instr_ptr = cast(self.mem[indices[1]]).ok_or("couldn't cast to usize")?;
                    return Ok(true);
                }
            }
            IntcodeOpcode::LessThan => {
                self.mem[indices[2]] = (self.mem[indices[0]] < self.mem[indices[1]]).into()
            }
            IntcodeOpcode::Equals => {
                self.mem[indices[2]] = (self.mem[indices[0]] == self.mem[indices[1]]).into();
            }
            IntcodeOpcode::RelBase => {
                self.relative_base += cast::<T, isize>(self.mem[indices[0]]).unwrap();
            }
            IntcodeOpcode::Halt => return Ok(false),
        };

        // Increment instruction pointer
        self.instr_ptr += 1 + opcode.num_of_params();
        Ok(true)
    }

    fn fetch_param_index(&self, index: usize, param_mode: &ParamMode) -> usize {
        match param_mode {
            ParamMode::PositionMode => cast(self.mem[index]).unwrap(),
            ParamMode::ImmediateMode => index,
            ParamMode::RelativeMode => {
                (self.relative_base + cast::<_, isize>(self.mem[index]).unwrap()) as usize
            }
        }
    }

    fn get_modes(ms: T, opcode: &IntcodeOpcode) -> Vec<ParamMode> {
        if opcode.num_of_params() == 0 {
            return Vec::new();
        }

        let mut modes: Vec<ParamMode> = format!("{:0params$}", ms, params = opcode.num_of_params())
            .chars()
            .map(|c| ParamMode::from_u32(c.to_digit(10).unwrap()).unwrap())
            .collect();
        modes.reverse();
        modes
    }

    fn read_line() -> i32 {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .map(|_| {
                buf.pop();
                buf.parse::<i32>()
            })
            .unwrap()
            .expect("invalid input to Input instruction")
    }

    fn send_output(&self, index: T, value: T) {
        match &self.outgoing {
            Some(sender) => sender.send((index, value)).unwrap(),
            _ => panic!("Can't send output - handlers haven't been configured"),
        }
    }

    fn receive_input(&self) -> T {
        match &self.incoming {
            Some(receiver) => receiver.recv().unwrap(),
            _ => panic!("Can't receive input - handlers haven't been configured"),
        }
    }
}

impl IntcodeOpcode {
    pub fn num_of_params(&self) -> usize {
        match self {
            Self::Add | Self::Mult | Self::LessThan | Self::Equals => 3,
            Self::JumpEq | Self::JumpNeq => 2,
            Self::Input | Self::Output | Self::RelBase => 1,
            Self::Halt => 0,
        }
    }
}
