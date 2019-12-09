//SOLUTIONS

#[aoc(day4, part1)]
pub fn part1(_: &str) -> i32 {
    let mut count = 0;

    for n in 254032..(789860 + 1) {
        let digits = to_digits(n);

        if digits_dont_decrease(digits) && min_two_adjacent_same(digits) {
            count += 1;
        }
    }

    count
}

#[aoc(day4, part2)]
pub fn part2(_raw_input: &str) -> i32 {
    let mut count = 0;

    for n in 254032..(789860 + 1) {
        let digits = to_digits(n);

        if digits_dont_decrease(digits) && exactly_two_adjacent_same(digits) {
            count += 1;
        }
    }

    count
}

// HELPERS
fn to_digits(n: i32) -> (i32, i32, i32, i32, i32, i32) {
    (
        n / 100000 % 10,
        n / 10000 % 10,
        n / 1000 % 10,
        n / 100 % 10,
        n / 10 % 10,
        n % 10,
    )
}

fn digits_dont_decrease(num: (i32, i32, i32, i32, i32, i32)) -> bool {
    num.0 <= num.1 && num.1 <= num.2 && num.2 <= num.3 && num.3 <= num.4 && num.4 <= num.5
}

fn min_two_adjacent_same(num: (i32, i32, i32, i32, i32, i32)) -> bool {
    num.0 == num.1 || num.1 == num.2 || num.2 == num.3 || num.3 == num.4 || num.4 == num.5
}

fn exactly_two_adjacent_same(num: (i32, i32, i32, i32, i32, i32)) -> bool {
    (num.0 == num.1 && num.1 != num.2)
        || (num.0 != num.1 && num.1 == num.2 && num.2 != num.3)
        || (num.1 != num.2 && num.2 == num.3 && num.3 != num.4)
        || (num.2 != num.3 && num.3 == num.4 && num.4 != num.5)
        || (num.3 != num.4 && num.4 == num.5)
}
