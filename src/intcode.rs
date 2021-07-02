use num_traits::FromPrimitive;

#[derive(Debug)]
pub struct IntcodeComputer {
    pub mem: Vec<i32>,
    instr_ptr: usize,
}

#[derive(Primitive)]
pub enum IntcodeOpcode {
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    Halt = 99,
}

#[derive(Primitive)]
enum ParamMode {
    PositionMode = 0,
    ImmediateMode = 1,
}

impl IntcodeComputer {
    pub fn new(mem: Vec<i32>) -> Self {
        IntcodeComputer { mem, instr_ptr: 0 }
    }

    pub fn run(self: &mut Self) {
        while self.execute_instruction() {}
    }

    fn execute_instruction(self: &mut Self) -> bool {
        let i = self.instr_ptr;

        // Get opcode and param indexes
        let opcode = IntcodeOpcode::from_i32(self.mem[i] % 100).unwrap();
        let modes = IntcodeComputer::get_modes(self.mem[i] / 100, &opcode);
        let indices: Vec<usize> = (i + 1..)
            .zip(modes.iter())
            .map(|(index, mode)| IntcodeComputer::fetch_param_index(&self.mem, index, mode))
            .collect();

        // Perform operation
        match opcode {
            IntcodeOpcode::Add => {
                self.mem[indices[2]] = self.mem[indices[0]] + self.mem[indices[1]];
            }
            IntcodeOpcode::Mult => {
                self.mem[indices[2]] = self.mem[indices[0]] * self.mem[indices[1]];
            }
            IntcodeOpcode::Halt => return false,
            IntcodeOpcode::Input => {
                println!("Input: ");
                self.mem[indices[0]] = IntcodeComputer::read_line();
            }
            IntcodeOpcode::Output => println!("Output: {}", indices[0]),
        };

        // Increment instruction pointer
        self.instr_ptr += 1 + opcode.num_of_params();
        true
    }

    fn fetch_param_index(memory: &Vec<i32>, index: usize, param_mode: &ParamMode) -> usize {
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
        std::io::stdin()
            .read_line(&mut buf)
            .map(|_| buf.parse::<i32>())
            .unwrap()
            .expect("invalid input to Input instruction")
    }
}

impl IntcodeOpcode {
    pub fn num_of_params(&self) -> usize {
        match self {
            IntcodeOpcode::Add | IntcodeOpcode::Mult => 3,
            IntcodeOpcode::Input | IntcodeOpcode::Output => 1,
            IntcodeOpcode::Halt => 0,
        }
    }
}
