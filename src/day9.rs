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
    let mut int_code = IntCode::init(program, vec![1]);

    int_code.run();

    int_code
        .output
        .iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join(", ")
}

#[aoc(day9, part2)]
pub fn part2(program: &[i64]) -> String {
    let mut int_code = IntCode::init(program, vec![2]);

    int_code.run();

    int_code
        .output
        .iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join(", ")
}

struct IntCode {
    memory: HashMap<i64, i64>,
    pc: i64,
    relative_offset: i64,
    input: Vec<i64>,
    output: Vec<i64>,
    done: bool,
}
impl IntCode {
    fn init(program: &[i64], input: Vec<i64>) -> IntCode {
        IntCode {
            memory: program
                .iter()
                .enumerate()
                .map(|(a, b)| (a as i64, *b))
                .into_iter()
                .collect(),
            pc: 0,
            relative_offset: 0,
            input: input,
            output: vec![],
            done: false,
        }
    }

    fn run(&mut self) {
        while !self.done {
            self.step();
        }
    }

    fn step(&mut self) {
        let op_code = self.memory[&self.pc];

        if self.done {
            println!("You are trying to step a done IntCode");
            unreachable!();
        }

        match to_digits(op_code) {
            (mode_t, mode_b, mode_a, 0, 1) => {
                let param_a: i64 = self.param_value(mode_a, 1);
                let param_b: i64 = self.param_value(mode_b, 2);
                let target_dir: i64 = self.dest_dir(mode_t, 3);
                self.memory.insert(target_dir, param_a + param_b);
                self.pc += 4;
            }
            (mode_t, mode_b, mode_a, 0, 2) => {
                let param_a: i64 = self.param_value(mode_a, 1);
                let param_b: i64 = self.param_value(mode_b, 2);
                let target_dir: i64 = self.dest_dir(mode_t, 3);
                self.memory.insert(target_dir, param_a * param_b);
                self.pc += 4;
            }
            (0, 0, mode, 0, 3) => {
                let target_dir: i64 = self.dest_dir(mode, 1);

                if let Some(val) = self.input.get(0) {
                    self.memory.insert(target_dir, *val);
                    self.input = (&(self.input[1..])).to_vec();
                    self.pc += 2;
                } else {
                    println!("need input");
                    unreachable!();
                }
            }
            (0, 0, mode, 0, 4) => {
                let val = self.param_value(mode, 1);
                self.output.push(val);
                self.pc += 2;
            }
            (0, mode_dest, mode_cond, 0, 5) => {
                let cond: i64 = self.param_value(mode_cond, 1);
                let dest: i64 = self.param_value(mode_dest, 2);

                self.pc = if cond != 0 { dest } else { self.pc + 3 };
            }
            (0, mode_dest, mode_cond, 0, 6) => {
                let cond: i64 = self.param_value(mode_cond, 1);
                let dest: i64 = self.param_value(mode_dest, 2);

                self.pc = if cond == 0 { dest } else { self.pc + 3 };
            }
            (mode_t, mode_b, mode_a, 0, 7) => {
                let param_a: i64 = self.param_value(mode_a, 1);
                let param_b: i64 = self.param_value(mode_b, 2);
                let target_dir: i64 = self.dest_dir(mode_t, 3);
                self.memory
                    .insert(target_dir, if param_a < param_b { 1 } else { 0 });
                self.pc += 4;
            }
            (mode_t, mode_b, mode_a, 0, 8) => {
                let param_a: i64 = self.param_value(mode_a, 1);
                let param_b: i64 = self.param_value(mode_b, 2);
                let target_dir: i64 = self.dest_dir(mode_t, 3);
                self.memory
                    .insert(target_dir, if param_a == param_b { 1 } else { 0 });
                self.pc += 4;
            }
            (0, 0, mode, 0, 9) => {
                self.relative_offset += self.param_value(mode, 1);
                self.pc += 2;
            }
            (0, 0, 0, 9, 9) => {
                self.done = true;
            }
            other => {
                println!("Invalid Instruction: {:?}", other);
                unreachable!()
            }
        }
    }

    fn param_value(&self, mode: i64, param_n: i64) -> i64 {
        let value = self.memory[&(self.pc + param_n)];
        let default: i64 = 0;

        match mode {
            0 => *(self.memory.get(&value).unwrap_or(&default)),
            1 => value,
            2 => self.memory[&(self.relative_offset + value)],
            _other => unreachable!(),
        }
    }

    fn dest_dir(&self, mode: i64, param_n: i64) -> i64 {
        let dir = self.memory[&(self.pc + param_n)];

        match mode {
            0 => dir,
            2 => self.relative_offset + dir,
            _other => unreachable!(),
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

#[cfg(test)]
mod tests {
    use super::part1;

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

}
