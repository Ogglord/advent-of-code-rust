use itertools::Itertools;
use std::collections::HashMap; // 0.8.0'

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 14), part_one, part_two)
}

// workaround for https://github.com/rust-lang/rust/issues/34511#issuecomment-373455139
trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

fn str_to_dict<'p, T: Iterator<Item = &'p str>>(
    iter: T,
) -> impl Iterator<Item = (&'p [u8], (&'p [u8; 2], &'p [u8; 2]))> + Captures<'p> {
    iter.map(|c| c.split_once(" -> ").unwrap()).map(|t| {
        (
            t.0[0..2].as_bytes(),
            (
                &[*t.0[0..1].as_bytes(), *t.1[0..1].as_bytes()],
                &[*t.0[0..1].as_bytes(), *t.1[0..1].as_bytes()],
            ),
        )
    })
}

pub fn part_one(input: &str) -> u64 {
    run2(input, 10).try_into().unwrap()
}

pub fn part_two(input: &str) -> u128 {
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
    fn test_part_one_real() {
        use aoc::read_file;
        let input = read_file("inputs", 14);
        assert_eq!(part_one(&input), 2703);
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

    let rules = input
        .lines()
        .filter(|&line| line.contains('>'))
        .map(|l| {
            let (k, v) = l.split_once(" -> ").unwrap();
            let (k, v) = (k.as_bytes(), v.as_bytes()[0]);
            ([k[0], k[1]], [k[0], v])
        })
        .collect::<Vec<_>>();

    rules.sort_unstable_by_key(|k| k.0);

    let rules_to_index = rules
        .iter()
        .map(|r| {
            (
                r.0,
                rules.binary_search_by_key(&r.1, |(k, v)| *k), // index of r.1
                rules.binary_search_by_key(&[r.1[1], r.0[1]], |(k, v)| *k), // index of "r.2"
            )
        })
        .collect::<Vec<_>>();

    let count = vec![0; rules.len()];
    for _ in 0..40 {}
        let newCount = 
    todo!() // return actual max-min
}
