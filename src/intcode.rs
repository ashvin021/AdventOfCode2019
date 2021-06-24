use num_traits::FromPrimitive;

#[derive(Debug)]
pub struct IntcodeComputer {
    pub memory: Vec<i32>,
    instr_ptr: usize,
}

#[derive(Primitive)]
pub enum IntcodeOpcode {
    Add = 1,
    Mult = 2,
    Halt = 99,
}

impl IntcodeComputer {
    pub fn new(memory: Vec<i32>) -> Self {
        IntcodeComputer {
            memory,
            instr_ptr: 0,
        }
    }

    pub fn run(self: &mut Self) {
        while self.execute_instruction() {}
    }

    fn execute_instruction(self: &mut Self) -> bool {
        let i = self.instr_ptr;

        let opcode = IntcodeOpcode::from_i32(self.memory[i]).unwrap();
        let op1 = self.memory[self.memory[i + 1] as usize];
        let op2 = self.memory[self.memory[i + 2] as usize];
        let save_index = self.memory[i + 3];

        let result = match opcode {
            IntcodeOpcode::Add => op1 + op2,
            IntcodeOpcode::Mult => op1 * op2,
            IntcodeOpcode::Halt => return false,
        };

        self.memory[save_index as usize] = result;
        self.instr_ptr += 4;
        true
    }
}
