use std::str::FromStr;

pub type Word = i64;
pub type Reference = i64;

/// An implemntation of an [Intcode](https://adventofcode.com/2019/day/2) computer
///
/// # Day 2 examples
/// ```
/// use common::computer::{Computer, Tape};
///
/// let mut computer = Computer::new();
///
/// computer.reset_and_load_tape(&"1,9,10,3,2,3,11,0,99,30,40,50".parse().unwrap());
/// assert_eq!(3500, computer.run());
///
/// computer.reset_and_load_tape(&"1,0,0,0,99".parse().unwrap());
/// assert_eq!(2, computer.run());
///
/// computer.reset_and_load_tape(&"2,3,0,3,99".parse().unwrap());
/// assert_eq!(2, computer.run());
/// assert_eq!(6, computer.memory.read_direct(3));
///
/// computer.reset_and_load_tape(&"2,4,4,5,99,0".parse().unwrap());
/// assert_eq!(2, computer.run());
/// assert_eq!(9801, computer.memory.read_direct(5));
///
/// computer.reset_and_load_tape(&"1,1,1,4,99,5,6,0,99".parse().unwrap());
/// assert_eq!(30, computer.run());
/// ```
///
/// # Day 5 examples
/// ```
/// use common::computer::{Computer, Tape};
///
/// let mut computer = Computer::new();
///
/// computer.reset_and_load_tape(&"1002,4,3,4,33".parse().unwrap());
/// assert_eq!(1002, computer.run());
/// ```
pub struct Computer {
    /// The memory state of the computer
    pub memory: Memory,
    /// The CPU state of the computer
    pub cpu: CPU,
    /// The input and output states
    pub io: IOStream,
    /// If debug mode is on, outputs things
    pub debug: bool,
}

impl Computer {
    /// Creates a new computer with a halting program
    pub fn new() -> Computer {
        Computer {
            memory: Memory { ram: vec![OpCode::Halt as Word], debug: false },
            cpu: CPU::new(),
            io: IOStream::new(),
            debug: false,
        }
    }

    /// Createas a new computer and initialises the memory from the
    /// contents of the tape
    pub fn new_with_tape(tape: &Tape) -> Computer {
        let mut c = Computer::new();
        c.load_tape(tape);
        return c;
    }

    /// Resets the computer and initialises the memory from the
    /// contents of the tape
    pub fn reset_and_load_tape(&mut self, tape: &Tape) {
        self.cpu.reset();
        self.io.reset();
        self.load_tape(tape);

    }

    fn load_tape(&mut self, tape: &Tape) {
        self.memory.ram = tape.contents.clone();
    }

    /// Runs the code in memory until it halts, returning the
    /// contents of memory location 0 after halting
    pub fn run(&mut self) -> Word {
        while self.cpu.step(&mut self.memory, &mut self.io, self.debug) == CPUState::AwaitingInstruction { };
        self.memory.read_direct(0)
    }

    pub fn cpu_state(&self) -> CPUState {
        self.cpu.state
    }
}

pub struct IOStream {
    pub debug: bool,

    input: Vec<Word>,
    pub output: Vec<Word>,
}

impl IOStream {
    fn new() -> IOStream {
        IOStream {
            input: Vec::new(),
            output: Vec::new(),
            debug: false,
        }
    }

    pub fn add_input(&mut self, value: Word) {
        self.input.insert(0,value);
    }

    pub fn reset(&mut self) {
        self.input = Vec::new();
        self.output = Vec::new();
    }

    fn consume(&mut self) -> Option<Word> {
        let n = self.input.pop();
        if self.debug {
            println!("> consume {:?}", n);
        }
        return n;
    }

    fn produce(&mut self, value: Word) {
        if self.debug {
            println!("< produce {}", value);
        }
        self.output.push(value);
    }
}

/// Memory from which to read and write
pub struct Memory {
    ram: Vec<Word>,
    debug: bool,
}

impl Memory {
    /// Reads the current value of the passed location from memory
    pub fn read_direct(&self, location: Reference) -> Word {
        if self.debug {
            println!("Reading from {}", location);
        }
        if location >= (self.ram.len() as Reference) {
            if self.debug {
                println!("Reading out of current bounds, returning 0");
            }
            return 0;
        }
        assert!(location >= 0);
        self.ram[location as usize]
    }

    /// Writes the passed value to the specified location in memory
    pub fn write_direct(&mut self, location: Reference, value: Word) {
        if self.debug {
            println!("Writing {} to {}", value, location);
        }
        if location >= (self.ram.len() as Reference) {
            if self.debug {
                println!("Reading out of current bounds, growing");
            }
            self.ram.resize((location + 1) as usize, 0);
        }
        assert!(location >= 0);
        self.ram[location as usize] = value;
    }

    /// Reads the value of the memory slot pointed to by the value in
    /// the passed location
    pub fn read_indirect(&self, location: Reference) -> Word {
        assert!(location >= 0);
        self.read_direct(self.ram[location as usize] as Reference)
    }

    /// Write the passed value to the slot in pointed to by the value
    /// in the passed location
    pub fn write_indirect(&mut self, location: Reference, value: Word) {
        assert!(location >= 0);
        self.write_direct(self.ram[location as usize] as Reference, value);
    }

    fn read(&self, location: Reference, mode: ParameterMode, cpu: &CPU) -> Word {
        match mode {
            ParameterMode::Position => self.read_indirect(location),
            ParameterMode::Immediate => self.read_direct(location),
            ParameterMode::Relative => {
                let offset = self.read_direct(location);
                let final_location = offset + cpu.relative_base;
                if self.debug {
                    println!("Reading relative. Base {}, offset {}, result {}", cpu.relative_base, offset, final_location);
                }
                self.read_direct(final_location)
            },
        }
    }

    fn write(&mut self, location: Reference, value: Word, mode: ParameterMode, cpu: &CPU) {
        match mode {
            ParameterMode::Position => self.write_indirect(location, value),
            ParameterMode::Immediate => self.write_direct(location, value),
            ParameterMode::Relative => {
                let offset = self.read_direct(location);
                let final_location = offset + cpu.relative_base;
                if self.debug {
                    println!("Reading relative. Base {}, offset {}, result {}", cpu.relative_base, offset, final_location);
                }
                self.write_direct(final_location, value)
            },
        };
    }
}

/// A tape representing the initial memory state of an Intcode computer
#[derive(Debug)]
pub struct Tape {
    pub contents: Vec<Word>
}

impl FromStr for Tape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.trim().split(",").map(|i| i.parse::<Word>().unwrap() ).collect();
        Ok(Tape { contents: data } )
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CPUState {
    AwaitingInstruction,
    AwaitingInput,
    Halted,
}

#[derive(Copy, Clone, Debug)]
enum OpCode {
    Add,
    Mul,

    ConsumeInput,
    ProduceOutput,

    JumpIfNotZero,
    JumpIfZero,

    LessThan,
    Equal,

    AdjustRelativeBase,

    Halt,
}

#[derive(Copy, Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn from_char(char: char) -> ParameterMode {
        match char {
            '1' => ParameterMode::Immediate,
            '2' => ParameterMode::Relative,
            _   => ParameterMode::Position,
        }
    }
}

pub struct CPU {
    instruction_pointer: Reference,
    state: CPUState,
    relative_base: Reference,
    last_instruction: Option<DecodedInstruction>,
}

#[derive(Debug, Copy, Clone)]
struct OpModes {
    p0_mode: ParameterMode,
    p1_mode: ParameterMode,
//    p2_mode: ParameterMode,
//    p3_mode: ParameterMode,
}

impl OpModes {
    fn new() -> OpModes {
        OpModes {
            p0_mode: ParameterMode::Position,
            p1_mode: ParameterMode::Position,
//            p2_mode: ParameterMode::Position,
//            p3_mode: ParameterMode::Position,
        }
    }
}

impl FromStr for OpModes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s.len() {
            4 => OpModes {
                p0_mode: ParameterMode::from_char(s.chars().nth(3).unwrap()),
                p1_mode: ParameterMode::from_char(s.chars().nth(2).unwrap()),
//                p2_mode: ParameterMode::from_char(s.chars().nth(1).unwrap()),
//                p3_mode: ParameterMode::from_char(s.chars().nth(0).unwrap()),
            },
            3 => OpModes {
                p0_mode: ParameterMode::from_char(s.chars().nth(2).unwrap()),
                p1_mode: ParameterMode::from_char(s.chars().nth(1).unwrap()),
//                p2_mode: ParameterMode::from_char(s.chars().nth(0).unwrap()),
//                p3_mode: ParameterMode::Position,
            },
            2 => OpModes {
                p0_mode: ParameterMode::from_char(s.chars().nth(1).unwrap()),
                p1_mode: ParameterMode::from_char(s.chars().nth(0).unwrap()),
//                p2_mode: ParameterMode::Position,
//                p3_mode: ParameterMode::Position,
            },
            1 => OpModes {
                p0_mode: ParameterMode::from_char(s.chars().nth(0).unwrap()),
                p1_mode: ParameterMode::Position,
//                p2_mode: ParameterMode::Position,
//                p3_mode: ParameterMode::Position,
            },
            _ => OpModes {
                p0_mode: ParameterMode::Position,
                p1_mode: ParameterMode::Position,
//                p2_mode: ParameterMode::Position,
//                p3_mode: ParameterMode::Position,
            },
        };

        Ok(result)
    }
}

#[derive(Debug, Copy, Clone)]
struct DecodedInstruction {
    operation: OpCode,
    modes: OpModes,
}

impl CPU {
    fn new() -> CPU {
        CPU { instruction_pointer: 0, state: CPUState::AwaitingInstruction, relative_base: 0, last_instruction: None }
    }

    fn reset(&mut self) {
        self.instruction_pointer = 0;
        self.state = CPUState::AwaitingInstruction;
        self.relative_base = 0;
        self.last_instruction = None;
    }

    fn execute_instruction(&mut self, memory: &mut Memory, io: &mut IOStream, debug: bool) -> CPUState {
        let location = self.consume_ip();
        let op = self.decode(memory.read_direct(location), debug);
        if debug {
            println!("STEP DECODED: {:?}", op);
        }

        self.last_instruction = Some(op.clone());

        match op.operation {
            OpCode::Add => self.op_add(memory, &op.modes, debug),
            OpCode::Mul => self.op_mul(memory, &op.modes, debug),
            OpCode::ConsumeInput => self.op_consume_input(memory, io, &op.modes, debug),
            OpCode::ProduceOutput => self.op_produce_output(memory, io, &op.modes, debug),
            OpCode::JumpIfNotZero => self.op_jump_not_zero(memory, &op.modes, debug),
            OpCode::JumpIfZero => self.op_jump_zero(memory, &op.modes, debug),
            OpCode::LessThan => self.op_less_than(memory, &op.modes, debug),
            OpCode::Equal => self.op_equal(memory, &op.modes, debug),
            OpCode::AdjustRelativeBase => self.op_adjust_relative_base(memory, &op.modes, debug),

            OpCode::Halt => CPUState::Halted,
        }
    }

    fn step(&mut self, memory: &mut Memory, io: &mut IOStream, debug: bool) -> CPUState {
        if debug {
            println!("STEP STATE: {:?}", self.state);
        }

        self.state = match self.state {
            CPUState::Halted => CPUState::Halted,
            CPUState::AwaitingInput => self.op_consume_input(memory, io, &self.last_instruction.unwrap().modes),
            CPUState::AwaitingInstruction => self.execute_instruction(memory, io, debug),
        };

        self.state
    }

    fn decode_op(op: Word) -> OpCode {
        match op {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::ConsumeInput,
            4 => OpCode::ProduceOutput,
            5 => OpCode::JumpIfNotZero,
            6 => OpCode::JumpIfZero,
            7 => OpCode::LessThan,
            8 => OpCode::Equal,
            9 => OpCode::AdjustRelativeBase,

            99 => OpCode::Halt,

            _ => panic!("Unrecognised instruction!"),
        }
    }

    fn decode(&self, instruction: Word, debug: bool) -> DecodedInstruction {
        if debug {
            println!("DECODE: {}", instruction);
        }

        let s = instruction.to_string();

        if s.len() <= 2 {
            return DecodedInstruction {
                operation: CPU::decode_op(instruction),
                modes: OpModes::new(),
            };
        }

        let operation = CPU::decode_op(s[s.len() - 2 .. ].parse().unwrap());
        let modes = s[ .. s.len() - 2].parse().unwrap();

        DecodedInstruction { operation, modes }
    }

    fn consume_ip(&mut self) -> Reference {
        self.instruction_pointer += 1;
        self.instruction_pointer - 1
    }

    fn op_add(&mut self, memory: &mut Memory, modes: &OpModes, debug: bool) -> CPUState {
        let param_a = self.consume_ip();
        let param_b= self.consume_ip();
        let param_c = self.consume_ip()

        if debug {
            println!("a: {}, b: {}, c: {}", param_a, param_b, param_c);
        }

        let a = memory.read(param_a, modes.p0_mode, &self);
        let b = memory.read(param_b, modes.p1_mode, &self);
        let result = a + b;

        if debug {
            println!("read {}, read {}, writing {}", a, b, result);
        }

        memory.write_indirect(param_c, result);

        CPUState::AwaitingInstruction
    }

    fn op_mul(&mut self, memory: &mut Memory, modes: &OpModes) -> CPUState {
        let a = memory.read(self.consume_ip(), modes.p0_mode, &self);
        let b = memory.read(self.consume_ip(), modes.p1_mode, &self);
        memory.write_indirect(self.consume_ip(), a * b);

        CPUState::AwaitingInstruction
    }

    fn op_consume_input(&mut self, memory: &mut Memory, io: &mut IOStream, modes: &OpModes) -> CPUState {
        let value = io.consume();
        if value.is_none() {
            return CPUState::AwaitingInput;
        }
        memory.write(self.consume_ip(), value.unwrap(), modes.p0_mode, &self);

        CPUState::AwaitingInstruction
    }

    fn op_produce_output(&mut self, memory: &mut Memory, io: &mut IOStream, modes: &OpModes) -> CPUState {
        let value = memory.read(self.consume_ip(), modes.p0_mode, &self);
        io.produce(value);

        CPUState::AwaitingInstruction
    }

    fn op_jump_not_zero(&mut self, memory: &mut Memory, modes: &OpModes) -> CPUState {
        let a = memory.read(self.consume_ip(), modes.p0_mode, &self);
        let b = memory.read(self.consume_ip(), modes.p1_mode, &self);
        if a != 0 {
            self.instruction_pointer = b as Reference;
        }

        CPUState::AwaitingInstruction
    }

    fn op_jump_zero(&mut self, memory: &mut Memory, modes: &OpModes) -> CPUState {
        let a = memory.read(self.consume_ip(), modes.p0_mode, &self);
        let b = memory.read(self.consume_ip(), modes.p1_mode, &self);
        if a == 0 {
            self.instruction_pointer = b as Reference;
        }

        CPUState::AwaitingInstruction
    }

    fn op_less_than(&mut self, memory: &mut Memory, modes: &OpModes) -> CPUState {
        let a = memory.read(self.consume_ip(), modes.p0_mode, &self);
        let b = memory.read(self.consume_ip(), modes.p1_mode, &self);
        let output =  if a < b {
            1
        } else {
            0
        };
        memory.write_indirect(self.consume_ip(), output);

        CPUState::AwaitingInstruction
    }

    fn op_equal(&mut self, memory: &mut Memory, modes: &OpModes) -> CPUState {
        let a = memory.read(self.consume_ip(), modes.p0_mode, &self);
        let b = memory.read(self.consume_ip(), modes.p1_mode, &self);
        let output =  if a == b {
            1
        } else {
            0
        };
        memory.write_indirect(self.consume_ip(), output);

        CPUState::AwaitingInstruction
    }

    fn op_adjust_relative_base(&mut self, memory: &mut Memory, modes: &OpModes, debug: bool) -> CPUState {
        let a = memory.read(self.consume_ip(), modes.p0_mode, &self);

        if debug {
            print!("Adjusting relative base {} by {} to ", self.relative_base, a);
        }

        self.relative_base = self.relative_base + a;

        if debug {
            println!("{}", self.relative_base);
        }

        CPUState::AwaitingInstruction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tape() {
        let tape: Tape = "1,2,3".parse().unwrap();
        assert_eq!(1, tape.contents[0]);
        assert_eq!(2, tape.contents[1]);
        assert_eq!(3, tape.contents[2]);
    }

    #[test]
    fn test_tape_with_whitespace_at_ends() {
        let tape: Tape = " 1,2,3 ".parse().unwrap();
        assert_eq!(1, tape.contents[0]);
        assert_eq!(2, tape.contents[1]);
        assert_eq!(3, tape.contents[2]);
    }

    #[test]
    fn test_day_02_a_works() {
        let input = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,6,19,1,9,19,23,2,23,10,27,1,27,5,31,1,31,6,35,1,6,35,39,2,39,13,43,1,9,43,47,2,9,47,51,1,51,6,55,2,55,10,59,1,59,5,63,2,10,63,67,2,9,67,71,1,71,5,75,2,10,75,79,1,79,6,83,2,10,83,87,1,5,87,91,2,9,91,95,1,95,5,99,1,99,2,103,1,103,13,0,99,2,14,0,0";
        let mut computer = Computer::new_with_tape(&input.parse().unwrap());
        computer.memory.write_direct(1, 12);
        computer.memory.write_direct(2, 2);
        let result = computer.run();
        assert_eq!(3931283, result);
    }

    #[test]
    fn test_example_1() {
        let mut computer = Computer::new();
        computer.load_tape(&"1,9,10,3,2,3,11,0,99,30,40,50".parse().unwrap());
        assert_eq!(3500, computer.run());
    }

    #[test]
    fn test_example_2() {
        let mut computer = Computer::new();
        computer.load_tape(&"1,0,0,0,99".parse().unwrap());
        assert_eq!(2, computer.run());
    }

    #[test]
    fn test_example_3() {
        let mut computer = Computer::new();
        computer.load_tape(&"2,3,0,3,99".parse().unwrap());
        assert_eq!(2, computer.run());
    }

    #[test]
    fn test_example_day5_a() {
        let mut computer = Computer::new();
        computer.load_tape(&"1002,4,3,4,33".parse().unwrap());
        assert_eq!(1002, computer.run());
    }

    #[test]
    fn test_example_day5_b_1() {
        let mut computer = Computer::new();
        let tape = "3,9,8,9,10,9,4,9,99,-1,8".parse().unwrap();

        computer.load_tape(&tape);
        computer.io.add_input(1);
        computer.run();
        assert_eq!(0, computer.io.output[0]);

        computer.reset_and_load_tape(&tape);
        computer.io.add_input(8);
        computer.run();
        assert_eq!(1, computer.io.output[0]);
    }

    #[test]
    fn test_example_day5_b_2() {
        let mut computer = Computer::new();
        let tape = "3,9,7,9,10,9,4,9,99,-1,8".parse().unwrap();

        computer.load_tape(&tape);
        computer.io.add_input(1);
        computer.run();
        assert_eq!(1, computer.io.output[0]);

        computer.reset_and_load_tape(&tape);
        computer.io.add_input(100);
        computer.run();
        assert_eq!(0, computer.io.output[0]);
    }

    #[test]
    fn test_example_day5_b_4() {
        let mut computer = Computer::new();
        let tape = "3,3,1108,-1,8,3,4,3,99".parse().unwrap();

        computer.load_tape(&tape);
        computer.io.add_input(1);
        computer.run();
        assert_eq!(0, computer.io.output[0]);

        computer.reset_and_load_tape(&tape);
        computer.io.add_input(8);
        computer.run();
        assert_eq!(1, computer.io.output[0]);
    }

    #[test]
    fn test_example_day5_b_5() {
        let mut computer = Computer::new();
        let tape = "3,3,1107,-1,8,3,4,3,99".parse().unwrap();

        computer.load_tape(&tape);
        computer.io.add_input(1);
        computer.run();
        assert_eq!(1, computer.io.output[0]);

        computer.reset_and_load_tape(&tape);
        computer.io.add_input(800);
        computer.run();
        assert_eq!(0, computer.io.output[0]);
    }

    #[test]
    fn test_example_day5_b_6() {
        let mut computer = Computer::new();
        let tape = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9".parse().unwrap();

        computer.load_tape(&tape);
        computer.io.add_input(0);
        computer.run();
        assert_eq!(0, computer.io.output[0]);

        computer.reset_and_load_tape(&tape);
        computer.io.add_input(800);
        computer.run();
        assert_eq!(1, computer.io.output[0]);
    }

    #[test]
    fn test_example_day5_b_7() {
        let mut computer = Computer::new();
        let tape = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1".parse().unwrap();

        computer.load_tape(&tape);
        computer.io.add_input(0);
        computer.run();
        assert_eq!(0, computer.io.output[0]);

        computer.reset_and_load_tape(&tape);
        computer.io.add_input(800);
        computer.run();
        assert_eq!(1, computer.io.output[0]);
    }

    #[test]
    fn test_example_day5_b_8() {
        let mut computer = Computer::new();
        let tape = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".parse().unwrap();

        computer.load_tape(&tape);
        computer.io.add_input(0);
        computer.run();
        assert_eq!(999, computer.io.output[0]);

        computer.reset_and_load_tape(&tape);
        computer.io.add_input(8);
        computer.run();
        assert_eq!(1000, computer.io.output[0]);

        computer.reset_and_load_tape(&tape);
        computer.io.add_input(800);
        computer.run();
        assert_eq!(1001, computer.io.output[0]);
    }

    #[test]
    fn test_example_day9_1() {
        let mut computer = Computer::new();
        let tape = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".parse().unwrap();

        computer.load_tape(&tape);
        computer.run();
        assert_eq!(tape.contents, computer.io.output);
    }

    #[test]
    fn test_example_day9_2() {
        let mut computer = Computer::new();
        let tape = "1102,34915192,34915192,7,4,7,99,0".parse().unwrap();

        computer.load_tape(&tape);
        computer.run();
        assert_eq!(1219070632396864, computer.io.output[0]);
    }

    #[test]
    fn test_example_day9_3() {
        let mut computer = Computer::new();
        let tape = "104,1125899906842624,99".parse().unwrap();

        computer.load_tape(&tape);
        computer.run();
        assert_eq!(1125899906842624, computer.io.output[0]);
    }
}
