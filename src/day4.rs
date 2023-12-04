use std::{collections::HashSet, fs};

fn parse_numbers(numbers_str: &str) -> HashSet<u32> {
    numbers_str
        .trim()
        .split(' ')
        .filter(|element| !element.is_empty())
        .map(|number| number.parse::<u32>().expect("Parsing error"))
        .collect()
}

fn part1(input: &str) -> u32 {
    let points: u32 = input
        .lines()
        .map(|line| line.get(9..).expect("Failed to trim"))
        .map(|line| line.split(" | ").collect())
        .map(|numbers: Vec<&str>| {
            let winning: HashSet<u32> = parse_numbers(numbers.first().expect("First item exists"));
            let found: HashSet<u32> = parse_numbers(numbers.last().expect("Last item exists"));

            let exponent = winning.intersection(&found).count();

            if exponent == 0 {
                return 0;
            }

            let mut points = 1;
            for _ in 0..winning.intersection(&found).count() - 1 {
                points *= 2;
            }

            return points;
        })
        .sum();

    return points;
}

pub fn day4() {
    let input: String =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("{}", part1(&input));
}
