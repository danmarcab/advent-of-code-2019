use itertools::Itertools;
use multimap::MultiMap;
use std::io;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|d| -> i32 { d.parse().unwrap() })
        .collect::<Vec<i32>>()
}

#[aoc(day7, part1)]
pub fn part1(program: &[i32]) -> i32 {
    let perms: Vec<Vec<i32>> = (0..5).permutations(5).collect();
    //    println!("{:?}", perms);
    //    println!("{:?}", perms.len());

    let mut results: MultiMap<Vec<i32>, i32> = MultiMap::new();

    for phase_sequence in perms {
        let res = run_sequence(program, &phase_sequence);

        results.insert(phase_sequence, res);
    }

    if let Some((seq, val)) = results.iter().max_by(|(k, v), (k2, v2)| v.cmp(v2)) {
        //        println!("{:?} -> {}", seq, val);
        *val
    } else {
        unreachable!()
    }

    //    let mut mut_program: Vec<i32> = program.into();
    //    run(&mut mut_program, 0);
    //
    //    mut_program[0]
}

fn run_sequence(program: &[i32], phase_sequence: &Vec<i32>) -> i32 {
    let mut output: Vec<i32> = Vec::new();
    let input: Vec<i32> = Vec::new();

    let mut input_signal = 0;

    for phase in phase_sequence {
        run_program(program, &vec![*phase, input_signal], &mut output);
        match output.last() {
            Some(val) => input_signal = *val,
            None => unreachable!(),
        }
    }

    input_signal
}

fn run_program(program: &[i32], input: &Vec<i32>, output: &mut Vec<i32>) {
    let mut mut_program: Vec<i32> = program.into();

    run(&mut mut_program, 0, &input, output);
}

#[aoc(day7, part2)]
pub fn part2(program: &[i32]) -> i32 {
    part1(program)
}

// HELPERS
fn to_digits(n: i32) -> (i32, i32, i32, i32, i32) {
    (
        n / 10000 % 10,
        n / 1000 % 10,
        n / 100 % 10,
        n / 10 % 10,
        n % 10,
    )
}

fn run(program: &mut Vec<i32>, pc: i32, input: &[i32], output: &mut Vec<i32>) {
    let op_code = program[pc as usize];

    //    println!("Executing {} at pos: {}", op_code, pc);
    //    println!("Program {:?}", program);

    match to_digits(op_code) {
        (0, mode_b, mode_a, 0, 1) => {
            let dest_dir: i32 = program[(pc + 3) as usize];
            let param_a: i32 = param_value(program, mode_a, pc + 1);
            let param_b: i32 = param_value(program, mode_b, pc + 2);
            program[dest_dir as usize] = param_a + param_b;
            //            println!("Adding: {} + {} into {}", param_a, param_b, dest_dir);
            run(program, pc + 4, input, output);
        }
        (0, mode_b, mode_a, 0, 2) => {
            let dest_dir: i32 = program[(pc + 3) as usize];
            let param_a: i32 = param_value(program, mode_a, pc + 1);
            let param_b: i32 = param_value(program, mode_b, pc + 2);
            program[dest_dir as usize] = param_a * param_b;
            //            println!("Multiplying: {} * {} into {}", param_a, param_b, dest_dir);
            run(program, pc + 4, input, output);
        }
        (0, 0, 0, 0, 3) => {
            let dest_dir: i32 = program[(pc + 1) as usize];

            //            println!("Enter int: ");int
            //            let mut buffer = String::new();
            //            io::stdin()
            //                .read_line(&mut buffer)
            //                .expect("failed to read input.");
            //            let n: i32 = buffer.trim().parse().expect("invalid input");

            program[dest_dir as usize] = input[0];
            run(program, pc + 2, &input[1..], output);
        }
        (0, 0, mode, 0, 4) => {
            //            println!("{}", param_value(program, mode, pc + 1));

            let val = param_value(program, mode, pc + 1);
            output.push(val);

            run(program, pc + 2, input, output);
        }
        (0, mode_dest, mode_cond, 0, 5) => {
            let cond: i32 = param_value(program, mode_cond, pc + 1);
            let dest: i32 = param_value(program, mode_dest, pc + 2);
            if cond != 0 {
                run(program, dest, input, output);
            } else {
                run(program, pc + 3, input, output);
            }
        }
        (0, mode_dest, mode_cond, 0, 6) => {
            let cond: i32 = param_value(program, mode_cond, pc + 1);
            let dest: i32 = param_value(program, mode_dest, pc + 2);
            if cond == 0 {
                run(program, dest, input, output);
            } else {
                run(program, pc + 3, input, output);
            }
        }
        (0, mode_b, mode_a, 0, 7) => {
            let dest_dir: i32 = program[(pc + 3) as usize];
            let param_a: i32 = param_value(program, mode_a, pc + 1);
            let param_b: i32 = param_value(program, mode_b, pc + 2);
            program[dest_dir as usize] = if param_a < param_b { 1 } else { 0 };
            run(program, pc + 4, input, output);
        }
        (0, mode_b, mode_a, 0, 8) => {
            let dest_dir: i32 = program[(pc + 3) as usize];
            let param_a: i32 = param_value(program, mode_a, pc + 1);
            let param_b: i32 = param_value(program, mode_b, pc + 2);
            program[dest_dir as usize] = if param_a == param_b { 1 } else { 0 };
            run(program, pc + 4, input, output);
        }
        (0, 0, 0, 9, 9) => {}
        other => {
            println!("Invalid Instruction: {:?}", other);
            unreachable!()
        }
    }
}

fn param_value(program: &Vec<i32>, mode: i32, dir: i32) -> i32 {
    let value = program[dir as usize];

    if mode == 1 {
        value
    } else if mode == 0 {
        program[value as usize]
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn day7part1() {
        assert_eq!(
            part1(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]),
            43210
        );
        assert_eq!(
            part1(&[
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ]),
            54321
        );
        assert_eq!(
            part1(&[
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ]),
            65210
        );
    }
}
