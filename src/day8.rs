use std::collections::HashSet;
use std::io;

#[derive(Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

struct ProgramState {
    idx: i32,
    acc_value: i32,
}

fn parse_instruction(line: &str) -> Instruction {
    let mut parts_iter = line.split_whitespace();

    let instr = parts_iter.next().unwrap();
    let value = parts_iter.next().unwrap().parse::<i32>().unwrap();

    match instr {
        "acc" => Instruction::Acc(value),
        "jmp" => Instruction::Jmp(value),
        "nop" => Instruction::Nop(value),
        _ => panic!("Got invalid instruction {}", instr),
    }
}

fn make_step(instrs: &Vec<Instruction>, state: &ProgramState) -> ProgramState {
    match instrs[state.idx as usize] {
        Instruction::Acc(v) => ProgramState {
            idx: state.idx + 1,
            acc_value: state.acc_value + v,
        },
        Instruction::Jmp(v) => ProgramState {
            idx: state.idx + v,
            acc_value: state.acc_value,
        },
        Instruction::Nop(_) => ProgramState {
            idx: state.idx + 1,
            acc_value: state.acc_value,
        },
    }
}

fn step_through_program(program: &Vec<Instruction>) -> bool {
    // step through program, return true if program terminates

    let mut visited: HashSet<i32> = HashSet::new();

    let mut state = ProgramState {
        idx: 0,
        acc_value: 0,
    };
    while !visited.contains(&state.idx) && state.idx < (program.len() as i32) {
        visited.insert(state.idx);

        state = make_step(&program, &state);
    }

    if visited.contains(&state.idx) {
        println!(
            "executing repeated instruction! accumulator: {}",
            state.acc_value
        );
        false
    } else {
        println!("final accumulator state: {}", state.acc_value);
        true
    }
}

fn day8b(program: &Vec<Instruction>) {
    // brute force: replace jmp/nop instructions one by one

    for (idx, instr) in program.iter().enumerate() {
        match instr {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(v) => {
                let mut new_program = (*program).clone();
                new_program[idx] = Instruction::Nop(*v);

                if step_through_program(&new_program) {
                    break;
                }
            }
            Instruction::Nop(v) => {
                let mut new_program = (*program).clone();
                new_program[idx] = Instruction::Jmp(*v);

                if step_through_program(&new_program) {
                    break;
                }
            }
        }
    }
}

pub fn day8(part_a: bool) {
    let mut program: Vec<Instruction> = Vec::new();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                program.push(parse_instruction(&line));
            }
        }
    }

    if part_a {
        step_through_program(&program);
    } else {
        day8b(&program);
    }
}
