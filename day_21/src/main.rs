use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = 
                "#ip 0\n\
                 seti 5 0 1\n\
                 seti 6 0 2\n\
                 addi 0 1 0\n\
                 addr 1 2 3\n\
                 setr 1 0 0\n\
                 seti 8 0 4\n\
                 seti 9 0 5";


    #[test]
    fn test_first_half() {
        let input = String::from(TEST_INPUT);
        let (ir, program) = parse_input(input);
        assert_eq!(run(ir, &program), [7, 5, 6, 0, 0, 9] );
    }
}

type Register = usize;
type RegisterFile = [Register; 6];

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

fn simulate(op: Operation, input: &mut RegisterFile) {
    input[op.c] = match op.opcode {
        Opcode::Add {imm} => if imm {input[op.a] + op.b as Register}
                                  else {input[op.a] + input[op.b]},
        Opcode::Mul {imm} => if imm {input[op.a] * op.b as Register}
                                  else {input[op.a] * input[op.b]},
        Opcode::And {imm} => if imm {input[op.a] & op.b as Register}
                                  else {input[op.a] & input[op.b]},
        Opcode::Or {imm}  => if imm {input[op.a] | op.b as Register}
                                  else {input[op.a] | input[op.b]},
        Opcode::Ass {imm} => if imm {op.a} else {input[op.a] as Register},
        Opcode::Gtir => (op.a as Register > input[op.b]) as Register,
        Opcode::Gtri => (input[op.a] > op.b as Register) as Register,
        Opcode::Gtrr => (input[op.a] > input[op.b]) as Register,
        Opcode::Eqir => (op.a as Register == input[op.b]) as Register,
        Opcode::Eqri => (input[op.a] == op.b as Register) as Register,
        Opcode::Eqrr => (input[op.a] == input[op.b]) as Register,
    };
}

#[derive(Clone, Copy, Debug)]
struct Operation {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

fn parse_insn(insn: &str) -> Operation {
    let opcode = match &insn[..4] {
        "addi" => Opcode::Add {imm: true},
        "addr" => Opcode::Add {imm: false},
        "muli" => Opcode::Mul {imm: true},
        "mulr" => Opcode::Mul {imm: false},
        "bani" => Opcode::And {imm: true},
        "banr" => Opcode::And {imm: false},
        "bori" => Opcode::Or  {imm: true},
        "borr" => Opcode::Or  {imm: false},
        "seti" => Opcode::Ass {imm: true},
        "setr" => Opcode::Ass {imm: false},
        "gtir" => Opcode::Gtir,
        "gtri" => Opcode::Gtri,
        "gtrr" => Opcode::Gtrr,
        "eqir" => Opcode::Eqir,
        "eqri" => Opcode::Eqri,
        "eqrr" => Opcode::Eqrr,
        _ => panic!("Unknown opcode: {}", &insn[..4]),
    };

    let mut vals = insn[5..].split(" ")
                            .map(|v| v.parse::<usize>().unwrap() );

    Operation {opcode: opcode,
               a: vals.next().unwrap(),
               b: vals.next().unwrap(),
               c: vals.next().unwrap()}
}

fn parse_input(input: String) -> (usize, Vec<Operation>) {
    let mut have_ir = false;
    let mut program = Vec::new();
    let mut ir = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if have_ir {
            program.push(parse_insn(line));
        } else {
            ir = line.split(" ")
                     .nth(1).unwrap()
                     .parse::<usize>().unwrap();
            have_ir = true;
        }
    }
    (ir as usize, program)
}

fn run(ir: usize, program: &Vec<Operation>, terminate: bool) -> Register {
    let mut registers = [0; 6];
    let mut vals = HashSet::new();
    let mut old_val = 0;
    loop {
        let opcode = program[ registers[ir] ];
        simulate(opcode, &mut registers);
        registers[ir] += 1;
        if registers[ir] >= program.len() {
            break;
        }
        if registers[ir] == program.len() - 1 {
            if terminate {
                return registers[2];
            }
            if vals.insert(registers[2]) {
                old_val = registers[2];
            } else {
                return old_val;
            }
        }
    }
    old_val
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let (ir, program) = parse_input(input);
    let register = run(ir, &program, true);
    println!("Third register contains {}", register);
    let register = run(ir, &program, false);
    println!("Third register contains {}", register);
}
