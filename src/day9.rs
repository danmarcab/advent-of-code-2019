use multimap::MultiMap;
use std::collections::HashMap;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|d| -> i64 { d.parse().unwrap() })
        .collect::<Vec<i64>>()
}

#[aoc(day9, part1)]
pub fn part1(program: &[i64]) -> String {
    let mut mut_program: HashMap<i64, i64> = program
        .iter()
        .enumerate()
        .map(|(a, b)| (a as i64, *b))
        .into_iter()
        .collect();
    let mut output: Vec<i64> = vec![];
    run(&mut mut_program, 0, 0, &[1], &mut output);

    println!("{:?}", output);

    output
        .iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join(", ")
}

#[aoc(day9, part2)]
pub fn part2(program: &[i64]) -> String {
    let mut mut_program: HashMap<i64, i64> = program
        .iter()
        .enumerate()
        .map(|(a, b)| (a as i64, *b))
        .into_iter()
        .collect();
    let mut output: Vec<i64> = vec![];
    run(&mut mut_program, 0, 0, &[2], &mut output);

    println!("{:?}", output);

    output
        .iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join(", ")
}

fn run(
    program: &mut HashMap<i64, i64>,
    pc: i64,
    relative_offset: i64,
    input: &[i64],
    output: &mut Vec<i64>,
) -> Option<i64> {
    let op_code = program[&pc];

    //    println!("Executing {} at pos: {}", op_code, pc);
    //        println!("Program {:?}", program);

    match to_digits(op_code) {
        (mode_t, mode_b, mode_a, 0, 1) => {
            let param_a: i64 = param_value(program, mode_a, pc + 1, relative_offset);
            let param_b: i64 = param_value(program, mode_b, pc + 2, relative_offset);
            let target_dir: i64 = dest_dir(program, mode_t, pc + 3, relative_offset);
            program.insert(target_dir, param_a + param_b);
            //            println!("Adding: {} + {} into {}", param_a, param_b, dest_dir);
            run(program, pc + 4, relative_offset, input, output)
        }
        (mode_t, mode_b, mode_a, 0, 2) => {
            let param_a: i64 = param_value(program, mode_a, pc + 1, relative_offset);
            let param_b: i64 = param_value(program, mode_b, pc + 2, relative_offset);
            let target_dir: i64 = dest_dir(program, mode_t, pc + 3, relative_offset);
            program.insert(target_dir, param_a * param_b);
            //            println!("Multiplying: {} * {} into {}", param_a, param_b, dest_dir);
            run(program, pc + 4, relative_offset, input, output)
        }
        (0, 0, mode, 0, 3) => {
            let target_dir: i64 = dest_dir(program, mode, pc + 1, relative_offset);

            //            println!("Enter int: ");int
            //            let mut buffer = String::new();
            //            io::stdin()
            //                .read_line(&mut buffer)
            //                .expect("failed to read input.");
            //            let n: i64 = buffer.trim().parse().expect("invalid input");

            if let Some(val) = input.get(0) {
                program.insert(target_dir, *val);
                run(program, pc + 2, relative_offset, &input[1..], output)
            } else {
                //                println!("suspending...");
                Some(pc)
            }
        }
        (0, 0, mode, 0, 4) => {
            //            println!("{}", param_value(program, mode, pc + 1, relative_offset));

            let val = param_value(program, mode, pc + 1, relative_offset);
            output.push(val);

            run(program, pc + 2, relative_offset, input, output)
        }
        (0, mode_dest, mode_cond, 0, 5) => {
            let cond: i64 = param_value(program, mode_cond, pc + 1, relative_offset);
            let dest: i64 = param_value(program, mode_dest, pc + 2, relative_offset);
            if cond != 0 {
                run(program, dest, relative_offset, input, output)
            } else {
                run(program, pc + 3, relative_offset, input, output)
            }
        }
        (0, mode_dest, mode_cond, 0, 6) => {
            let cond: i64 = param_value(program, mode_cond, pc + 1, relative_offset);
            let dest: i64 = param_value(program, mode_dest, pc + 2, relative_offset);
            if cond == 0 {
                run(program, dest, relative_offset, input, output)
            } else {
                run(program, pc + 3, relative_offset, input, output)
            }
        }
        (mode_t, mode_b, mode_a, 0, 7) => {
            let param_a: i64 = param_value(program, mode_a, pc + 1, relative_offset);
            let param_b: i64 = param_value(program, mode_b, pc + 2, relative_offset);
            let target_dir: i64 = dest_dir(program, mode_t, pc + 3, relative_offset);
            program.insert(target_dir, if param_a < param_b { 1 } else { 0 });
            run(program, pc + 4, relative_offset, input, output)
        }
        (mode_t, mode_b, mode_a, 0, 8) => {
            let param_a: i64 = param_value(program, mode_a, pc + 1, relative_offset);
            let param_b: i64 = param_value(program, mode_b, pc + 2, relative_offset);
            let target_dir: i64 = dest_dir(program, mode_t, pc + 3, relative_offset);
            program.insert(target_dir, if param_a == param_b { 1 } else { 0 });
            run(program, pc + 4, relative_offset, input, output)
        }
        (0, 0, mode, 0, 9) => {
            let new_offset = relative_offset + param_value(program, mode, pc + 1, relative_offset);
            run(program, pc + 2, new_offset, input, output)
        }
        (0, 0, 0, 9, 9) => None,
        other => {
            println!("Invalid Instruction: {:?}", other);
            unreachable!()
        }
    }
}
// HELPERS
fn to_digits(n: i64) -> (i64, i64, i64, i64, i64) {
    (
        n / 10000 % 10,
        n / 1000 % 10,
        n / 100 % 10,
        n / 10 % 10,
        n % 10,
    )
}

fn param_value(program: &HashMap<i64, i64>, mode: i64, dir: i64, relative_offset: i64) -> i64 {
    let value = program[&dir];
    let default: i64 = 0;

    match mode {
        0 => *(program.get(&value).unwrap_or(&default)),
        1 => value,
        2 => program[&(relative_offset + value)],

        other => unreachable!(),
    }
}

fn dest_dir(program: &HashMap<i64, i64>, mode: i64, dir: i64, relative_offset: i64) -> i64 {
    let dir = program[&dir];

    match mode {
        0 => dir,
        2 => relative_offset + dir,
        other => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn day9part1() {
        assert_eq!(
            part1(&[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]),
            "109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99"
        );
        assert_eq!(
            part1(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0]),
            "1219070632396864"
        );
        assert_eq!(part1(&[104, 1125899906842624, 99]), "1125899906842624");
    }
    //    #[test]
    //    fn day9part2() {
    //        assert_eq!(
    //            part2(&[
    //                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
    //                -1, 28, 1005, 28, 6, 99, 0, 0, 5
    //            ]),
    //            139629729
    //        );
    //        assert_eq!(
    //            part1(&[
    //                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
    //                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
    //                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
    //            ]),
    //            18216
    //        );
    //    }
}
