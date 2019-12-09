#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|d| -> usize { d.parse().unwrap() })
        .collect::<Vec<usize>>()
}

#[aoc(day2, part1)]
pub fn part1(program: &[usize]) -> usize {
    let mut mut_program: Vec<usize> = program.into();
    mut_program[1] = 12;
    mut_program[2] = 2;
    run(&mut mut_program, 0);

    mut_program[0]
}

fn run(program: &mut Vec<usize>, pc: usize) {
    let op_code = program[pc];

    match op_code {
        1 => {
            let dest: usize = program[pc + 3];
            let op1: usize = program[program[pc + 1]];
            let op2: usize = program[program[pc + 2]];
            program[dest] = op1 + op2;
            run(program, pc + 4);
        }
        2 => {
            let dest: usize = program[pc + 3];
            let op1: usize = program[program[pc + 1]];
            let op2: usize = program[program[pc + 2]];
            program[dest] = op1 * op2;
            run(program, pc + 4);
        }
        99 => {}
        _ => unreachable!(),
    }
}

#[aoc(day2, part2)]
pub fn part2(input: &[usize]) -> usize {
    trywith(input, 0, 0)
}

fn trywith(program: &[usize], noun: usize, verb: usize) -> usize {
    let mut mut_program: Vec<usize> = program.into();
    mut_program[1] = noun;
    mut_program[2] = verb;
    run(&mut mut_program, 0);

    if mut_program[0] == 19690720 {
        100 * noun + verb
    } else if verb == 99 {
        trywith(program, noun + 1, 0)
    } else {
        trywith(program, noun, verb + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn sample1() {
        assert_eq!(part1(&[1, 0, 0, 0, 99]), 2);
    }
}
