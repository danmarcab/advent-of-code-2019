use multimap::MultiMap;

fn prepare_input(raw_input: &str) -> Vec<(&str, &str)> {
    raw_input
        .trim()
        .split('\n')
        .map(|orbit| -> (&str, &str) {
            match orbit.split(')').collect::<Vec<&str>>().as_slice() {
                [obj_a, obj_b] => (obj_a, obj_b),
                other => {
                    println!("{:?}", other);
                    unreachable!()
                }
            }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(raw_input: &str) -> usize {
    let pairs = prepare_input(raw_input);

    let mut direct_orbits: MultiMap<&str, &str> = MultiMap::new();

    pairs.iter().fold(
        &mut direct_orbits,
        |map: &mut MultiMap<&str, &str>, orbit: &(&str, &str)| {
            map.insert(orbit.0, orbit.1);
            map
        },
    );

    calc_orbits(&direct_orbits, "COM", 0)
}

fn calc_orbits(direct_orbits: &MultiMap<&str, &str>, key: &str, level: usize) -> usize {
    let from_children = match direct_orbits.get_vec(key) {
        Some(vec) => vec
            .iter()
            .map(|child_key| calc_orbits(direct_orbits, child_key, level + 1))
            .sum(),
        None => 0,
    };

    level + from_children
}

#[aoc(day6, part2)]
pub fn part2(raw_input: &str) -> usize {
    let pairs = prepare_input(raw_input);

    let mut rev_direct_orbits: MultiMap<&str, &str> = MultiMap::new();

    pairs.iter().fold(
        &mut rev_direct_orbits,
        |map: &mut MultiMap<&str, &str>, orbit: &(&str, &str)| {
            map.insert(orbit.1, orbit.0);
            map
        },
    );

    let mut path_to_you = Vec::new();
    let mut path_to_santa = Vec::new();

    get_path(&rev_direct_orbits, "YOU", &mut path_to_you);
    get_path(&rev_direct_orbits, "SAN", &mut path_to_santa);

    path_to_you.reverse();
    path_to_santa.reverse();
    calc_steps_to_common_obj(&path_to_you, &path_to_santa)
}

fn calc_steps_to_common_obj(path1: &Vec<&str>, path2: &Vec<&str>) -> usize {
    let mut common_steps = 0;

    while path1[common_steps] == path2[common_steps] {
        common_steps += 1
    }

    path1.len() + path2.len() - common_steps - common_steps
}

fn get_path<'a>(orbits: &MultiMap<&str, &'a str>, key: &str, path: &mut Vec<&'a str>) {
    match orbits.get(key) {
        Some(&"COM") => {}
        Some(&prev_key) => {
            path.push(prev_key);
            get_path(orbits, prev_key, path);
        }
        None => unreachable!(),
    };
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn day6part1() {
        assert_eq!(
            part1(
                "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
            ),
            42
        );
    }

    #[test]
    fn day6part2() {
        assert_eq!(
            part2(
                "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"
            ),
            4
        );
    }
}
