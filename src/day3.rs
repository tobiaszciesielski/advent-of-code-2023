use std::fs;

#[derive(Debug)]
struct Number {
    x_start: usize,
    x_end: usize,
    y: usize,
    value: u32,
}

#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
}

fn extract_numbers(input: &str) -> Vec<Number> {
    let mut prepared_numbers: Vec<Number> = Vec::new();

    input.lines().enumerate().for_each(|(y, line)| {
        let lines_with_numbers: Vec<char> = line
            .chars()
            .map(|char| {
                if char.is_ascii_alphanumeric() {
                    char
                } else {
                    '.'
                }
            })
            .collect();

        let tmp = String::from_iter(lines_with_numbers);
        let numbers: Vec<&str> = tmp.split(".").collect();

        let mut x = 0;
        numbers.iter().for_each(|number| {
            if x == line.len() {
                return;
            }

            if !number.is_empty() {
                let parsed_number: u32 = number.parse().expect("Error with parsing number");

                prepared_numbers.push(Number {
                    x_start: x,
                    x_end: x + number.len() - 1,
                    y: y,
                    value: parsed_number,
                });

                x += number.len()
            }

            x += 1;
        });
    });

    return prepared_numbers;
}

fn extract_symbols(input: &str) -> Vec<Symbol> {
    let mut prepared_symbols: Vec<Symbol> = Vec::new();

    input.lines().enumerate().for_each(|(y, line)| {
        let lines_with_symbols: Vec<char> = line
            .chars()
            .map(|char| {
                if !char.is_ascii_alphanumeric() {
                    char
                } else {
                    '.'
                }
            })
            .collect();

        lines_with_symbols
            .iter()
            .enumerate()
            .for_each(|(x, symbol)| {
                if symbol != &'.' {
                    prepared_symbols.push(Symbol { x: x, y: y });
                }
            });
    });

    return prepared_symbols;
}

pub fn part1(input: &str) -> u32 {
    let numbers = extract_numbers(input);
    let symbols = extract_symbols(input);

    let mut sum = 0;
    for number in numbers {
        for symbol in &symbols {
            if [symbol.y - 1, symbol.y, symbol.y + 1].contains(&number.y) {
                if symbol.x - 1 >= number.x_start && symbol.x - 1 <= number.x_end
                    || symbol.x >= number.x_start && symbol.x <= number.x_end
                    || symbol.x + 1 >= number.x_start && symbol.x + 1 <= number.x_end
                {
                    sum += number.value;
                }
            }
        }
    }

    return sum;
}

pub fn day3() {
    let input: String =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("{}", part1(&input));
}
