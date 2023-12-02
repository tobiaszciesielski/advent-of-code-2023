use std::fs;

pub fn part1(input: &str) -> u32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut game_sum = 0;

    input.lines().for_each(|line| {
        let splitted_line: Vec<&str> = line.split(':').collect();

        let game_id: u32 = splitted_line
            .first()
            .expect("Game not exists")
            .get(5..)
            .expect("Failed to get id")
            .parse()
            .expect("Failed to parse");

        let mut sets: Vec<&str> = splitted_line
            .last()
            .expect("Game sets not exists")
            .split(';')
            .collect();

        let bool_sets: Vec<bool> = sets
            .iter_mut()
            .map(|set| {
                let bool_set: Vec<bool> = set
                    .split(',')
                    .map(|pick| {
                        let cubes_amount: u32 = pick
                            .get(1..3)
                            .expect("Cubes not exists")
                            .trim()
                            .parse()
                            .expect("Failed to parse");

                        let too_much_cubes: bool =
                            match pick.get(3..).expect("Cubes not exists").trim() {
                                "red" => cubes_amount > max_red,
                                "green" => cubes_amount > max_green,
                                "blue" => cubes_amount > max_blue,
                                _ => true,
                            };

                        return too_much_cubes;
                    })
                    .collect();

                return bool_set.contains(&true);
            })
            .collect();

        if !bool_sets.contains(&true) {
            game_sum += game_id;
        }
    });

    return game_sum;
}

pub fn part2(input: &str) -> u32 {
    let cubes_multiplication: Vec<u32> = input.lines().map(|line| {
        let splitted_line: Vec<&str> = line.split(':').collect();

        let game_id: u32 = splitted_line
            .first()
            .expect("Game not exists")
            .get(5..)
            .expect("Failed to get id")
            .parse()
            .expect("Failed to parse");

        let mut sets: Vec<&str> = splitted_line
            .last()
            .expect("Game sets not exists")
            .split(';')
            .collect();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let bool_sets: Vec<bool> = sets
            .iter_mut()
            .map(|set| {
                let bool_set: Vec<bool> = set
                    .split(',')
                    .map(|pick| {
                        let cubes_amount: u32 = pick
                            .get(1..3)
                            .expect("Cubes not exists")
                            .trim()
                            .parse()
                            .expect("Failed to parse");

                        match pick.get(3..).expect("Cubes not exists").trim() {
                            "red" => {
                                if cubes_amount > max_red {
                                    max_red = cubes_amount
                                }
                            }
                            "green" => {
                                if cubes_amount > max_green {
                                    max_green = cubes_amount
                                }
                            }
                            "blue" => {
                                if cubes_amount > max_blue {
                                    max_blue = cubes_amount
                                }
                            }
                            _ => {}
                        };

                        return false;
                    })
                    .collect();

                return bool_set.contains(&true);
            })
            .collect();

        return max_blue * max_green * max_red;
    }).collect();

    return cubes_multiplication.iter().sum();
}

pub fn day2() {
    let input =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
