#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.trim().parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    input.iter().fold(0, |sum, w| sum + (w / 3 - 2))
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    input.iter().fold(0, |sum, w| sum + fuel_for(*w))
}

fn fuel_for(weight: u32) -> u32 {
    let fuel = weight / 3 - 2;

    if fuel < 9 {
        fuel
    } else {
        fuel + fuel_for(fuel)
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn sample1() {
        assert_eq!(part1(&[12]), 2);
        assert_eq!(part1(&[14]), 2);
        assert_eq!(part1(&[1969]), 654);
        assert_eq!(part1(&[100756]), 33583);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&[14]), 2);
        assert_eq!(part2(&[1969]), 966);
        assert_eq!(part2(&[100756]), 50346);
    }
}
