pub fn part_one(input: &str) -> u32 {
    0
}

pub fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 15), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 15);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 15);
        assert_eq!(part_two(&input), 0);
    }
}
