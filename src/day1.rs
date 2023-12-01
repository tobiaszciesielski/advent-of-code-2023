use std::fs;

pub fn day1() {
    let mut contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    contents = contents
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    let mut sum = 0;
    for line in contents.split('\n') {
        let filtered_chars: Vec<char> = line.chars().filter(|x| x.is_ascii_digit()).collect();

        if filtered_chars.len() == 0 {
            continue;
        }

        let digit = format!(
            "{}{}",
            filtered_chars.first().expect("Exists"),
            filtered_chars.last().expect("Exists")
        );

        sum += digit.parse::<i32>().unwrap();
    }

    println!("{}", sum)
}
