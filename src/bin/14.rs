use itertools::Itertools;
use std::collections::HashMap; // 0.8.0

fn str_to_tuple<'a, T: Iterator<Item = &'a str>>(iter: T) -> impl Iterator<Item = (&'a str, char)> {
    iter.map(|c| c.split_once(" -> ").unwrap())
        .map(|t| (t.0, t.1.chars().next().unwrap()))
}

pub fn part_one(input: &str) -> u32 {
    let mut key = String::from(input.lines().next().unwrap());

    let key_value_pairs = input.lines().filter(|&line| line.contains(">"));
    // sort them into a hashmap (AB -> C) etc...
    let m: HashMap<_, _> = str_to_tuple(key_value_pairs).into_iter().collect();

    // perform the replacements, 10 times
    for i in 0..10 {
        let mut v = Vec::new();
        let mut final_char = char::MAX;
        for (a, b) in key.chars().tuple_windows() {
            v.push(a);
            v.push(m[&String::from_iter([a, b]) as &str]);
            final_char = b;
        }
        v.push(final_char);
        key = String::from_iter(v);
    }
    println!("After final iteration length: {}", key.len());
    let hm = key.chars().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c).or_insert(0) += 1;
        hm
    });

    // find the least + most common character (min, max)
    let mut min_max = (0, 0);
    for (_, val) in hm.iter() {
        if *val > min_max.1 {
            min_max.1 = *val;
        }
        if *val < min_max.0 || min_max.0 == 0 {
            min_max.0 = *val;
        }
    }
    min_max.1 - min_max.0
}

pub fn part_two(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 14);
        assert_eq!(part_one(&input), 1588);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 14);
        assert_eq!(part_two(&input), 0);
    }
}
