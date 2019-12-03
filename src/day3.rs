use std::collections::HashMap;
use std::str::FromStr;

//PARSING

#[derive(Debug, PartialEq, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
struct Segment {
    dir: Dir,
    length: i32,
}

impl FromStr for Dir {
    type Err = std::string::String;

    fn from_str(dir_str: &str) -> Result<Self, Self::Err> {
        match dir_str {
            "U" => Ok(Dir::Up),
            "D" => Ok(Dir::Down),
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            other => Err(format!("Invalid Dir: {}", other)),
        }
    }
}

impl FromStr for Segment {
    type Err = std::string::String;

    fn from_str(segment_str: &str) -> Result<Self, Self::Err> {
        let dir = segment_str[0..1].parse::<Dir>()?;
        let length = segment_str[1..]
            .parse::<i32>()
            .map_err(|_| format!("Invalid Dir: {}", segment_str))?;

        Ok(Segment {
            dir: dir,
            length: length,
        })
    }
}

type Cable = Vec<Segment>;

fn prepare_input(raw_input: &str) -> Input {
    let cables: Vec<Cable> = raw_input
        .trim()
        .split('\n')
        .map(|cable_str| -> Cable {
            cable_str
                .split(',')
                .map(|d| -> Segment { d.parse().unwrap() })
                .collect()
        })
        .collect();

    match cables.as_slice() {
        [cable_a, cable_b] => Input {
            cable_a: cable_a.to_vec(),
            cable_b: cable_b.to_vec(),
        },
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Input {
    cable_a: Cable,
    cable_b: Cable,
}

//SOLUTIONS

#[aoc(day3, part1)]
pub fn part1(raw_input: &str) -> i32 {
    let crossings = calc_crossings(prepare_input(raw_input));

    let closest = crossings
        .iter()
        .min_by(|cross, cross2| manhattan_dist(cross).cmp(&manhattan_dist(cross2)))
        .unwrap();

    manhattan_dist(closest)
}

#[aoc(day3, part2)]
pub fn part2(raw_input: &str) -> i32 {
    let crossings = calc_crossings(prepare_input(raw_input));

    let earliest = crossings
        .iter()
        .min_by(|cross, cross2| cross.steps.cmp(&cross2.steps))
        .unwrap();

    earliest.steps
}

//HELPERS

fn calc_crossings(input: Input) -> Vec<Crossing> {
    let mut grid: Grid = HashMap::new();

    lay_out_cable(&mut grid, input.cable_a);
    cross_cable(&grid, input.cable_b)
}

type Steps = i32;

type Grid = HashMap<Position, Steps>;

#[derive(Debug, PartialEq, Copy, Eq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}
struct Crossing {
    x: i32,
    y: i32,
    steps: i32,
}

fn manhattan_dist(crossing: &Crossing) -> i32 {
    crossing.x.abs() + crossing.y.abs()
}

fn lay_out_cable(grid: &mut Grid, cable: Cable) {
    cable.iter().fold(
        (Position { x: 0, y: 0 }, 0),
        |acc: (Position, Steps), segment: &Segment| {
            let mut new_pos = acc.0;
            let mut new_steps = acc.1;

            for _ in 1..(segment.length + 1) {
                new_pos = move_to(new_pos, &segment.dir);
                new_steps += 1;
                grid.insert(new_pos, new_steps);
            }

            (new_pos, new_steps)
        },
    );
}
fn cross_cable(grid: &Grid, cable: Cable) -> Vec<Crossing> {
    let mut crossings: Vec<Crossing> = Vec::new();
    cable.iter().fold(
        (Position { x: 0, y: 0 }, 0),
        |acc: (Position, Steps), segment: &Segment| {
            let mut new_pos = acc.0;
            let mut new_steps = acc.1;

            for _ in 1..(segment.length + 1) {
                new_pos = move_to(new_pos, &segment.dir);
                new_steps += 1;
                match grid.get(&new_pos) {
                    Some(steps) => {
                        crossings.push(Crossing {
                            x: new_pos.x,
                            y: new_pos.y,
                            steps: steps + new_steps,
                        });
                    }
                    None => {}
                }
            }

            (new_pos, new_steps)
        },
    );

    crossings
}

fn move_to(pos: Position, dir: &Dir) -> Position {
    match dir {
        Dir::Up => Position {
            y: pos.y + 1,
            ..pos
        },
        Dir::Down => Position {
            y: pos.y - 1,
            ..pos
        },
        Dir::Left => Position {
            x: pos.x - 1,
            ..pos
        },
        Dir::Right => Position {
            x: pos.x + 1,
            ..pos
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn day3part1() {
        assert_eq!(
            part1(
                "R8,U5,L5,D3
U7,R6,D4,L4"
            ),
            6
        );
        assert_eq!(
            part1(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
        assert_eq!(
            part1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn day3part2() {
        assert_eq!(
            part2(
                "R8,U5,L5,D3
U7,R6,D4,L4"
            ),
            30
        );
        assert_eq!(
            part2(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            610
        );
        assert_eq!(
            part2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
