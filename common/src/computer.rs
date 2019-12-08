use std::str::FromStr;

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
            memory: Memory { ram: vec![OpCode::Halt as i32] },
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
    pub fn run(&mut self) -> i32 {
        while self.cpu.step(&mut self.memory, &mut self.io, self.debug) != StepResult::Halt { };
        self.memory.read_direct(0)
    }
}

pub struct IOStream {
    input: Vec<i32>,
    pub output: Vec<i32>,
}

impl IOStream {
    fn new() -> IOStream {
        IOStream {
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    pub fn add_input(&mut self, value: i32) {
        self.input.push(value);
    }

    pub fn reset(&mut self) {
        self.input = Vec::new();
        self.output = Vec::new();
    }

    fn consume(&mut self) -> i32 {
        self.input.pop().unwrap()
    }

    fn produce(&mut self, value: i32) {
        self.output.push(value);
    }
}

/// Memory from which to read and write
pub struct Memory {
    ram: Vec<i32>,
}

impl Memory {
    /// Reads the current value of the passed location from memory
    pub fn read_direct(&self, location: usize) -> i32 {
        self.ram[location]
    }

    /// Writes the passed value to the specified location in memory
    pub fn write_direct(&mut self, location: usize, value: i32) {
        self.ram[location] = value;
    }

    /// Reads the value of the memory slot pointed to by the value in
    /// the passed location
    pub fn read_indirect(&self, location: usize) -> i32 {
        self.read_direct(self.ram[location] as usize)
    }

    /// Write the passed value to the slot in pointed to by the value
    /// in the passed location
    pub fn write_indirect(&mut self, location: usize, value: i32) {
        self.write_direct(self.ram[location] as usize, value);
    }

    fn read(&self, location: usize, mode: ParameterMode) -> i32 {
        match mode {
            ParameterMode::Position => self.read_indirect(location),
            ParameterMode::Immediate => self.read_direct(location),
        }
    }
}

/// A tape representing the initial memory state of an Intcode computer
pub struct Tape {
    contents: Vec<i32>
}

impl FromStr for Tape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.trim().split(",").map(|i| i.parse::<i32>().unwrap() ).collect();
        Ok(Tape { contents: data } )
    }
}

#[derive(PartialEq)]
enum StepResult {
    Continue,
    Halt,
}

#[derive(Copy, Clone, Debug)]
enum OpCode {
    Add,
    Mul,

    ReadR1,
    WriteR1,

    JumpIfNotZero,
    JumpIfZero,

    LessThan,
    Equal,

    Halt,
}

#[derive(Copy, Clone, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn from_char(char: char) -> ParameterMode {
        match char {
            '1' => ParameterMode::Immediate,
            _   => ParameterMode::Position,
        }
    }
}

pub struct CPU {
    instruction_pointer: usize,
}

#[derive(Debug)]
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

#[derive(Debug)]
struct DecodedInstruction {
    operation: OpCode,
    modes: OpModes,
}

impl CPU {
    fn new() -> CPU {
        CPU { instruction_pointer: 0, }
    }

    fn reset(&mut self) {
        self.instruction_pointer = 0;
    }

    fn step(&mut self, memory: &mut Memory, io: &mut IOStream, debug: bool) -> StepResult {
        let location = self.consume_ip();
        let op = self.decode(memory.read_direct(location), debug);
        if debug {
            println!("STEP: {:?}", op);
        }

        match op.operation {
            OpCode::Add => self.op_add(memory, &op.modes),
            OpCode::Mul => self.op_mul(memory, &op.modes),
            OpCode::ReadR1 => self.op_read_r1(memory, io),
            OpCode::WriteR1 => self.op_write_r1(memory, io, &op.modes),
            OpCode::JumpIfNotZero => self.op_jump_not_zero(memory, &op.modes),
            OpCode::JumpIfZero => self.op_jump_zero(memory, &op.modes),
            OpCode::LessThan => self.op_less_than(memory, &op.modes),
            OpCode::Equal => self.op_equal(memory, &op.modes),

            OpCode::Halt => return StepResult::Halt,
        };

        return StepResult::Continue;
    }

    fn decode_op(op: i32) -> OpCode {
        match op {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            3 => OpCode::ReadR1,
            4 => OpCode::WriteR1,
            5 => OpCode::JumpIfNotZero,
            6 => OpCode::JumpIfZero,
            7 => OpCode::LessThan,
            8 => OpCode::Equal,

            99 => OpCode::Halt,

            _ => panic!("Unrecognised instruction!"),
        }
    }

    fn decode(&self, instruction: i32, debug: bool) -> DecodedInstruction {
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

    fn consume_ip(&mut self) -> usize {
        self.instruction_pointer += 1;
        self.instruction_pointer - 1
    }

    fn op_add(&mut self, memory: &mut Memory, modes: &OpModes) {
        let a = memory.read(self.consume_ip(), modes.p0_mode);
        let b = memory.read(self.consume_ip(), modes.p1_mode);
        memory.write_indirect(self.consume_ip(), a + b);
    }

    fn op_mul(&mut self, memory: &mut Memory, modes: &OpModes) {
        let a = memory.read(self.consume_ip(), modes.p0_mode);
        let b = memory.read(self.consume_ip(), modes.p1_mode);
        memory.write_indirect(self.consume_ip(), a * b);
    }

    fn op_read_r1(&mut self, memory: &mut Memory, io: &mut IOStream) {
        let value = io.consume();
        memory.write_indirect(self.consume_ip(), value);
    }

    fn op_write_r1(&mut self, memory: &mut Memory, io: &mut IOStream, modes: &OpModes) {
        let value = memory.read(self.consume_ip(), modes.p0_mode);
        io.produce(value);
    }

    fn op_jump_not_zero(&mut self, memory: &mut Memory, modes: &OpModes) {
        let a = memory.read(self.consume_ip(), modes.p0_mode);
        let b = memory.read(self.consume_ip(), modes.p1_mode);
        if a != 0 {
            self.instruction_pointer = b as usize;
        }
    }

    fn op_jump_zero(&mut self, memory: &mut Memory, modes: &OpModes) {
        let a = memory.read(self.consume_ip(), modes.p0_mode);
        let b = memory.read(self.consume_ip(), modes.p1_mode);
        if a == 0 {
            self.instruction_pointer = b as usize;
        }
    }

    fn op_less_than(&mut self, memory: &mut Memory, modes: &OpModes) {
        let a = memory.read(self.consume_ip(), modes.p0_mode);
        let b = memory.read(self.consume_ip(), modes.p1_mode);
        let output =  if a < b {
            1
        } else {
            0
        };
        memory.write_indirect(self.consume_ip(), output);
    }

    fn op_equal(&mut self, memory: &mut Memory, modes: &OpModes) {
        let a = memory.read(self.consume_ip(), modes.p0_mode);
        let b = memory.read(self.consume_ip(), modes.p1_mode);
        let output =  if a == b {
            1
        } else {
            0
        };
        memory.write_indirect(self.consume_ip(), output);
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
}
