use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashSet, HashMap};

type Register = u32;
type RegisterFile = [Register; 4];
type Instruction = [u32; 4];

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum Opcode {
    Add { imm: bool },
    Mul { imm: bool },
    And { imm: bool },
    Or  { imm: bool },
    Ass { imm: bool },
    Gtir, Gtri, Gtrr,
    Eqir, Eqri, Eqrr,
}

impl Opcode {
    fn from(int: i32) -> Opcode {
        match int {
            0  => Opcode::Add { imm: true },
            1  => Opcode::Add { imm: false },
            2  => Opcode::Mul { imm: true },
            3  => Opcode::Mul { imm: false },
            4  => Opcode::And { imm: true },
            5  => Opcode::And { imm: false },
            6  => Opcode::Or  { imm: true },
            7  => Opcode::Or  { imm: false },
            8  => Opcode::Ass { imm: true },
            9  => Opcode::Ass { imm: false },
            10 => Opcode::Gtir,
            11 => Opcode::Gtri,
            12 => Opcode::Gtrr,
            13 => Opcode::Eqir,
            14 => Opcode::Eqri,
            15 => Opcode::Eqrr,
            _ => panic!("Too high an int for opcode: {}", int),
        }
    }
}

fn simulate(op: Operation, input: RegisterFile) -> RegisterFile {
    let mut output = input;
    output[op.c] = match op.opcode {
        Opcode::Add {imm} => if imm {input[op.a] + op.b as Register}
                                  else {input[op.a] + input[op.b]},
        Opcode::Mul {imm} => if imm {input[op.a] * op.b as Register}
                                  else {input[op.a] * input[op.b]},
        Opcode::And {imm} => if imm {input[op.a] & op.b as Register}
                                  else {input[op.a] & input[op.b]},
        Opcode::Or {imm}  => if imm {input[op.a] | op.b as Register}
                                  else {input[op.a] | input[op.b]},
        Opcode::Ass {imm} => if imm {input[op.a]} else {op.a as Register},
        Opcode::Gtir => (op.a as Register > input[op.b]) as Register,
        Opcode::Gtri => (input[op.a] > op.b as Register) as Register,
        Opcode::Gtrr => (input[op.a] > input[op.b]) as Register,
        Opcode::Eqir => (op.a as Register == input[op.b]) as Register,
        Opcode::Eqri => (input[op.a] == op.b as Register) as Register,
        Opcode::Eqrr => (input[op.a] == input[op.b]) as Register,
    };
    output
}

#[derive(Clone, Copy)]
struct Operation {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

struct Example {
    input: RegisterFile,
    output: RegisterFile,
    instruction: Instruction,
}

fn parse_insn(insn: &str) -> Instruction {
    let mut vals = insn.split(" ")
        .map(|v| v.parse::<u32>().unwrap());

    [vals.next().unwrap(),
     vals.next().unwrap(),
     vals.next().unwrap(),
     vals.next().unwrap()]
}

fn parse_rf(reg: &str) -> RegisterFile {
    let mut vals = reg.split(", ")
        .map(|v| v.parse::<u32>().unwrap());

    [vals.next().unwrap(),
     vals.next().unwrap(),
     vals.next().unwrap(),
     vals.next().unwrap()]
}

fn parse_input(input: String) -> (Vec<Example>, Vec<Instruction>) {
    let mut before = None;
    let mut insn = None;
    let mut last_newline = false;
    let mut reached_program = false;
    let mut program = Vec::new();
    let mut examples = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            if last_newline == true {
                reached_program = true;
            }
            last_newline = true;
            continue;
        }
        last_newline = false;

        if reached_program {
            program.push(parse_insn(line));
        } else {
            if line.find("Before:") == Some(0) {
                before = Some(parse_rf(&line[9..19]));
            } else if line.find("After:") == Some(0) {
                let after = parse_rf(&line[9..19]);
                examples.push( Example{ input: before.unwrap(),
                                        output: after,
                                        instruction: insn.unwrap() });
            } else {
                insn = Some(parse_insn(line));
            }

        }
    }
    (examples, program)
}

fn get_aliases(ex: &Example) -> HashSet<Opcode> {
    let mut aliases = HashSet::new();

    for i in 0..16 {
        let oper = Operation { opcode: Opcode::from(i),
                               a: ex.instruction[1] as usize,
                               b: ex.instruction[2] as usize,
                               c: ex.instruction[3] as usize };
        let output = simulate(oper.clone(), ex.input);
        if output == ex.output {
            aliases.insert(oper.opcode);
        }
    }
    aliases
}

fn count_aliases(examples: &Vec<Example>) -> u32 {
    let mut num_aliases = 0;
    for example in examples {
        if get_aliases(example).len() >= 3 {
            num_aliases += 1;
        }
    }
    num_aliases
}


fn solve_operations(examples: &Vec<Example>) -> [Opcode; 16] {
    let mut possibilites: HashMap<u32, HashSet<Opcode>> = HashMap::new();

    let mut taken = HashSet::new();
    let mut finished = HashSet::new();

    for example in examples {
        if finished.contains(&example.instruction[0]) {
            continue;
        }

        let aliases = get_aliases(example);
        let aliases: HashSet<Opcode> = aliases.difference(&taken)
                                              .map(|a| *a).collect();
        let intersection =
            if possibilites.contains_key(&example.instruction[0]) {
                possibilites.get(&example.instruction[0]).unwrap()
                            .intersection(&aliases)
                            .map(|a| *a).collect()
            } else {
                aliases
            };
        if intersection.len() == 1 {
            taken.insert(intersection.iter().next().unwrap().clone());
            finished.insert(example.instruction[0]);
        }
        possibilites.insert(example.instruction[0], intersection);
    }

    if possibilites.len() != 16 {
        panic!("Didn't resolve all instructions");
    }

    let mut operations = [Opcode::Add { imm: true }; 16];
    for (key, opcodes) in possibilites {
        assert!(opcodes.len() == 1, "{} aliases for opc {}",
                opcodes.len(), key);
        operations[key as usize] = opcodes.iter().next().unwrap().clone();
    }
    operations
}

fn run_program(operations: [Opcode; 16], program: Vec<Instruction>)
    -> RegisterFile {
        let mut registers = [0, 0, 0, 0];
        for insn in program {
            let oper = Operation { opcode: operations[insn[0] as usize],
                                   a: insn[1] as usize,
                                   b: insn[2] as usize,
                                   c: insn[3] as usize };
            registers = simulate(oper, registers);
        }
        registers
    }

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let (examples, program) = parse_input(input);

    let aliases = count_aliases(&examples);
    println!("Aliases: {}", aliases);

    let oper_map = solve_operations(&examples);
    let end_state = run_program(oper_map, program);
    println!("Register end state: {} {} {} {}", end_state[0], end_state[1],
                                                end_state[2], end_state[3]);
}
