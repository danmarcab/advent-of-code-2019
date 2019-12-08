use itertools::Itertools;
use multimap::MultiMap;
use std::collections::HashMap;
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

    let mut results: MultiMap<Vec<i32>, i32> = MultiMap::new();

    for phase_sequence in perms {
        let res = run_sequence(program, &phase_sequence);

        results.insert(phase_sequence, res);
    }

    if let Some((_seq, val)) = results.iter().max_by(|(_k, v), (_k2, v2)| v.cmp(v2)) {
        //        println!("{:?} -> {}", seq, val);
        *val
    } else {
        unreachable!()
    }
}

fn run_sequence(program: &[i32], phase_sequence: &Vec<i32>) -> i32 {
    let mut programs: Vec<Vec<i32>> = vec![];
    let mut inputs: Vec<Vec<i32>> = vec![];
    let mut outputs: Vec<Vec<i32>> = vec![];
    let mut pcs: Vec<i32> = vec![];

    let mut finished = false;

    // initial values
    for i in 0..5 {
        programs.push(program.into());
        inputs.push(vec![phase_sequence[i]]);
        outputs.push(Vec::new());
        pcs.push(0);
    }
    // initial input
    inputs[0].push(0);

    let mut iters = 1;
    while !finished {
        for i in 0..5 {
            println!("Executing iter {} fuse {}", iters, i);
            println!("I: {:?}", inputs);
            println!("O: {:?}", outputs);
            println!("pcs: {:?}", pcs);
            println!("");

            match run(&mut programs[i], pcs[i], &inputs[i], &mut outputs[i]) {
                Some(new_pc) => {
                    pcs[i] = new_pc;
                    inputs[i] = vec![];
                }
                None => {
                    finished = true;
                }
            }

            match outputs[i].last() {
                Some(val) => inputs[(i + 1) % 5].push(*val),
                None => unreachable!(),
            }
        }
        iters += 1;
    }
    println!("Iters {:?}", iters);
    println!("I: {:?}", inputs);
    println!("O: {:?}", outputs);
    println!("pcs: {:?}", pcs);

    *outputs[4].last().unwrap()
}

#[aoc(day7, part2)]
pub fn part2(program: &[i32]) -> i32 {
    let perms: Vec<Vec<i32>> = (5..10).permutations(5).collect();

    //    let perms: Vec<Vec<i32>> = vec![vec![5, 6, 7, 8, 9]];

    let mut results: MultiMap<Vec<i32>, i32> = MultiMap::new();

    for phase_sequence in perms {
        let res = run_sequence(program, &phase_sequence);

        results.insert(phase_sequence, res);
    }

    if let Some((_seq, val)) = results.iter().max_by(|(_k, v), (_k2, v2)| v.cmp(v2)) {
        //        println!("{:?} -> {}", seq, val);
        *val
    } else {
        unreachable!()
    }
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

fn run(program: &mut Vec<i32>, pc: i32, input: &[i32], output: &mut Vec<i32>) -> Option<i32> {
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
            run(program, pc + 4, input, output)
        }
        (0, mode_b, mode_a, 0, 2) => {
            let dest_dir: i32 = program[(pc + 3) as usize];
            let param_a: i32 = param_value(program, mode_a, pc + 1);
            let param_b: i32 = param_value(program, mode_b, pc + 2);
            program[dest_dir as usize] = param_a * param_b;
            //            println!("Multiplying: {} * {} into {}", param_a, param_b, dest_dir);
            run(program, pc + 4, input, output)
        }
        (0, 0, 0, 0, 3) => {
            let dest_dir: i32 = program[(pc + 1) as usize];

            //            println!("Enter int: ");int
            //            let mut buffer = String::new();
            //            io::stdin()
            //                .read_line(&mut buffer)
            //                .expect("failed to read input.");
            //            let n: i32 = buffer.trim().parse().expect("invalid input");

            if let Some(val) = input.get(0) {
                program[dest_dir as usize] = *val;
                run(program, pc + 2, &input[1..], output)
            } else {
                //                println!("suspending...");
                Some(pc)
            }
        }
        (0, 0, mode, 0, 4) => {
            //            println!("{}", param_value(program, mode, pc + 1));

            let val = param_value(program, mode, pc + 1);
            output.push(val);

            run(program, pc + 2, input, output)
        }
        (0, mode_dest, mode_cond, 0, 5) => {
            let cond: i32 = param_value(program, mode_cond, pc + 1);
            let dest: i32 = param_value(program, mode_dest, pc + 2);
            if cond != 0 {
                run(program, dest, input, output)
            } else {
                run(program, pc + 3, input, output)
            }
        }
        (0, mode_dest, mode_cond, 0, 6) => {
            let cond: i32 = param_value(program, mode_cond, pc + 1);
            let dest: i32 = param_value(program, mode_dest, pc + 2);
            if cond == 0 {
                run(program, dest, input, output)
            } else {
                run(program, pc + 3, input, output)
            }
        }
        (0, mode_b, mode_a, 0, 7) => {
            let dest_dir: i32 = program[(pc + 3) as usize];
            let param_a: i32 = param_value(program, mode_a, pc + 1);
            let param_b: i32 = param_value(program, mode_b, pc + 2);
            program[dest_dir as usize] = if param_a < param_b { 1 } else { 0 };
            run(program, pc + 4, input, output)
        }
        (0, mode_b, mode_a, 0, 8) => {
            let dest_dir: i32 = program[(pc + 3) as usize];
            let param_a: i32 = param_value(program, mode_a, pc + 1);
            let param_b: i32 = param_value(program, mode_b, pc + 2);
            program[dest_dir as usize] = if param_a == param_b { 1 } else { 0 };
            run(program, pc + 4, input, output)
        }
        (0, 0, 0, 9, 9) => None,
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
    #[test]
    fn day7part2() {
        assert_eq!(
            part2(&[
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ]),
            139629729
        );
        assert_eq!(
            part1(&[
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ]),
            18216
        );
    }
}
