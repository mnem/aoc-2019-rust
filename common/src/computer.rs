pub struct Computer {
    pub memory: Memory,
    cpu: CPU,
}

impl Computer {
    pub fn new_with_tape(tape: &Vec<i32>) -> Computer {
        Computer {
            memory: Memory { ram: tape.clone() },
            cpu: CPU { instruction_pointer: 0 },
        }
    }

    pub fn load_tape(&mut self, tape: &Vec<i32>) {
        self.memory.ram = tape.clone();
    }

    pub fn run(&mut self) -> i32 {
        println!("Memory: {:?}", self.memory.ram);
        while self.cpu.step(&mut self.memory) != StepResult::Halt {
            println!("Memory: {:?}", self.memory.ram);
        };
        self.memory.read(0)
    }
}

pub struct Memory {
    ram: Vec<i32>,
}

impl Memory {
    pub fn read(&self, location: usize) -> i32 {
        self.ram[location]
    }

    pub fn write(&mut self, location: usize, value: i32) {
        self.ram[location] = value;
    }

    pub fn read_indirect(&self, location: usize) -> i32 {
        self.read(self.ram[location] as usize)
    }

    pub fn write_indirect(&mut self, location: usize, value: i32) {
        self.write(self.ram[location] as usize, value);
    }
}

#[derive(PartialEq)]
enum StepResult {
    Continue,
    Halt,
}

enum OpCode {
    Add = 1,
    Mul = 2,

    Halt = 99,
}

struct CPU {
    instruction_pointer: usize,
}

struct DecodedInstruction {
    operation: OpCode,
}

impl CPU {
    fn step(&mut self, memory: &mut Memory) -> StepResult {
        let location = self.consume_ip();
        let op = self.decode(memory.read(location));
        match op.operation {
            OpCode::Add => self.op_add(memory),
            OpCode::Mul => self.op_mul(memory),

            OpCode::Halt => return StepResult::Halt,
        };

        return StepResult::Continue;
    }

    fn decode(&self, instruction: i32) -> DecodedInstruction {
        match instruction {
            1  => DecodedInstruction { operation: OpCode::Add },
            2  => DecodedInstruction { operation: OpCode::Mul },
            99 => DecodedInstruction { operation: OpCode::Halt },
            _  => panic!("Unrecognised instruction!"),
        }
    }

    fn consume_ip(&mut self) -> usize {
        self.instruction_pointer += 1;
        self.instruction_pointer - 1
    }

    fn op_add(&mut self, memory: &mut Memory) {
        let a = memory.read_indirect(self.consume_ip());
        let b = memory.read_indirect(self.consume_ip());
        memory.write_indirect(self.consume_ip(), a + b);
    }

    fn op_mul(&mut self, memory: &mut Memory) {
        let a = memory.read_indirect(self.consume_ip());
        let b = memory.read_indirect(self.consume_ip());
        memory.write_indirect(self.consume_ip(), a * b);
    }

}
