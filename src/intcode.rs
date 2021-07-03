use std::io;

use crossbeam::channel::{unbounded, Receiver, Sender};
use num_traits::FromPrimitive;

#[derive(Debug)]
pub struct IntcodeComputer {
    pub mem: Vec<i32>,
    instr_ptr: usize,
    incoming: Option<Receiver<i32>>,
    outgoing: Option<Sender<(i32, i32)>>,
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
    Halt = 99,
}

#[derive(Primitive)]
enum ParamMode {
    PositionMode = 0,
    ImmediateMode = 1,
}

impl IntcodeComputer {
    pub fn new(mem: Vec<i32>) -> Self {
        IntcodeComputer {
            mem,
            instr_ptr: 0,
            incoming: None,
            outgoing: None,
        }
    }

    pub fn with_io(mem: Vec<i32>) -> (Self, Sender<i32>, Receiver<(i32, i32)>) {
        let (s_input, r_input) = unbounded();
        let (s_output, r_output) = unbounded();
        (
            IntcodeComputer {
                mem,
                instr_ptr: 0,
                incoming: Some(r_input),
                outgoing: Some(s_output),
            },
            s_input,
            r_output,
        )
    }

    pub fn run(&mut self) {
        while self.execute_instruction() {}
    }

    fn execute_instruction(&mut self) -> bool {
        let i = self.instr_ptr;

        // Get opcode and param indexes
        let opcode = IntcodeOpcode::from_i32(self.mem[i] % 100).unwrap();
        let modes = Self::get_modes(self.mem[i] / 100, &opcode);
        let indices: Vec<usize> = (i + 1..)
            .zip(modes.iter())
            .map(|(index, mode)| Self::fetch_param_index(&self.mem, index, mode))
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
            IntcodeOpcode::Output => self.send_output(i as i32, self.mem[indices[0]]),
            IntcodeOpcode::JumpEq => {
                if self.mem[indices[0]] != 0 {
                    self.instr_ptr = self.mem[indices[1]] as usize;
                    return true;
                }
            }
            IntcodeOpcode::JumpNeq => {
                if self.mem[indices[0]] == 0 {
                    self.instr_ptr = self.mem[indices[1]] as usize;
                    return true;
                }
            }
            IntcodeOpcode::LessThan => {
                self.mem[indices[2]] = (self.mem[indices[0]] < self.mem[indices[1]]) as i32;
            }
            IntcodeOpcode::Equals => {
                self.mem[indices[2]] = (self.mem[indices[0]] == self.mem[indices[1]]) as i32;
            }
            IntcodeOpcode::Halt => return false,
        };

        // Increment instruction pointer
        self.instr_ptr += 1 + opcode.num_of_params();
        true
    }

    fn fetch_param_index(memory: &[i32], index: usize, param_mode: &ParamMode) -> usize {
        match param_mode {
            ParamMode::PositionMode => memory[index] as usize,
            ParamMode::ImmediateMode => index,
        }
    }

    fn get_modes(ms: i32, opcode: &IntcodeOpcode) -> Vec<ParamMode> {
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

    fn send_output(&self, index: i32, value: i32) {
        match &self.outgoing {
            Some(sender) => sender.send((index, value)).unwrap(),
            _ => panic!("Can't send output - handlers haven't been configured"),
        }
    }

    fn receive_input(&self) -> i32 {
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
            Self::Input | Self::Output => 1,
            Self::Halt => 0,
        }
    }
}
