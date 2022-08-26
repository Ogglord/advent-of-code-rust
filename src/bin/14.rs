use itertools::Itertools;
use std::collections::HashMap; // 0.8.0'

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 14), part_one, part_two)
}

// workaround for https://github.com/rust-lang/rust/issues/34511#issuecomment-373455139
trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

fn str_to_char<'p, T: Iterator<Item = &'p str>>(
    iter: T,
) -> impl Iterator<Item = ((char, char), char)> + Captures<'p> {
    iter.map(|c| c.split_once(" -> ").unwrap()).map(|t| {
        (
            t.0.chars().collect_tuple::<(char, char)>().unwrap(),
            t.1.chars().next().unwrap(),
        )
    })
}

pub fn part_one(input: &str) -> u64 {
    run2(input, 10).try_into().unwrap()
}

pub fn part_two(input: &str) -> u64 {
    run2(input, 40).try_into().unwrap()
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
        assert_eq!(part_two(&input), 2188189693529);
    }
}

fn run2(input: &str, iterations: u32) -> u128 {
    let key = String::from(input.lines().next().unwrap());

    let key_value_pairs_str = input.lines().filter(|&line| line.contains('>'));
    // sort them into a hashmap (AB -> C) etc...
    let key_value_pairs: HashMap<_, _> = str_to_char(key_value_pairs_str).into_iter().collect();

    let mut status: HashMap<_, u128> = key
        .chars()
        .tuple_windows::<(_, _)>()
        .map(|p| (p, 1))
        .collect();

    //println!("{:?}", status);

    for _i in 0..iterations {
        let iter_vector = status.into_iter();
        let mut updated_status = HashMap::new();
        for (a, b) in iter_vector {
            let c = key_value_pairs[&a];
            *updated_status.entry((a.0, c)).or_insert(0) += b;
            *updated_status.entry((c, a.1)).or_insert(0) += b;
        }

        status = updated_status;
        //println!("After iteration {} polymer length: {}", i, status.len());
        //println!("{}", DeepSizeOf::deep_size_of(&status));
    }

    let mut char_counts: HashMap<char, u128> = HashMap::new();

    for (k, v) in status {
        *char_counts.entry(k.0).or_insert(0) += v;
    }

    let max = char_counts.values().max().unwrap() + 1; // we know we had a trailing B, missing...;
    let min = char_counts.values().min().unwrap();

    max - min
}
