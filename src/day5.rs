use std::io;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|d| -> i32 { d.parse().unwrap() })
        .collect::<Vec<i32>>()
}

#[aoc(day5, part1)]
pub fn part1(program: &[i32]) -> i32 {
    let mut mut_program: Vec<i32> = program.into();
    run(&mut mut_program, 0);

    mut_program[0]
}

#[aoc(day5, part2)]
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

fn run(program: &mut Vec<i32>, pc: i32) {
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
            run(program, pc + 4);
        }
        (0, mode_b, mode_a, 0, 2) => {
            let dest_dir: i32 = program[(pc + 3) as usize];
            let param_a: i32 = param_value(program, mode_a, pc + 1);
            let param_b: i32 = param_value(program, mode_b, pc + 2);
            program[dest_dir as usize] = param_a * param_b;
            //            println!("Multiplying: {} * {} into {}", param_a, param_b, dest_dir);
            run(program, pc + 4);
        }
        (0, 0, 0, 0, 3) => {
            let dest_dir: i32 = program[(pc + 1) as usize];

            print!("Enter int: ");
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("failed to read input.");
            let n: i32 = buffer.trim().parse().expect("invalid input");

            program[dest_dir as usize] = n;
            run(program, pc + 2);
        }
        (0, 0, mode, 0, 4) => {
            println!("{}", param_value(program, mode, pc + 1));

            run(program, pc + 2);
        }
        (0, mode_dest, mode_cond, 0, 5) => {
            let cond: i32 = param_value(program, mode_cond, pc + 1);
            let dest: i32 = param_value(program, mode_dest, pc + 2);
            if cond != 0 {
                run(program, dest);
            } else {
                run(program, pc + 3)
            }
        }
        (0, mode_dest, mode_cond, 0, 6) => {
            let cond: i32 = param_value(program, mode_cond, pc + 1);
            let dest: i32 = param_value(program, mode_dest, pc + 2);
            if cond == 0 {
                run(program, dest);
            } else {
                run(program, pc + 3)
            }
        }
        (0, mode_b, mode_a, 0, 7) => {
            let dest_dir: i32 = program[(pc + 3) as usize];
            let param_a: i32 = param_value(program, mode_a, pc + 1);
            let param_b: i32 = param_value(program, mode_b, pc + 2);
            program[dest_dir as usize] = if param_a < param_b { 1 } else { 0 };
            run(program, pc + 4);
        }
        (0, mode_b, mode_a, 0, 8) => {
            let dest_dir: i32 = program[(pc + 3) as usize];
            let param_a: i32 = param_value(program, mode_a, pc + 1);
            let param_b: i32 = param_value(program, mode_b, pc + 2);
            program[dest_dir as usize] = if param_a == param_b { 1 } else { 0 };
            run(program, pc + 4);
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
}
