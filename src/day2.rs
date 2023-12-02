use std::fs::File;
use std::io;
use std::io::BufRead;

trait AppendWith<T> {
    fn append(&self, value: T) -> Self;
}

impl AppendWith<u32> for u32 {
    fn append(&self, value: u32) -> Self {
        self * 10 + value
    }
}

struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}

impl Sample {
    fn new() -> Self {
        Sample {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn is_possible(&self, max: &Sample) -> bool {
        self.red <= max.red && self.green <= max.green && self.blue <= max.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: u32,
    samples: Vec<Sample>,
}

impl Game {
    fn parse(s: String) -> Result<Game, &'static str> {
        let mut game_id: u32 = 0;
        let mut samples: Vec<Sample> = Vec::new();

        let mut current_num: u32 = 0;
        let mut current_sample: Sample = Sample::new();
        for c in s.strip_prefix("Game ").unwrap().chars() {
            match c {
                digit if c.is_digit(10) => {
                    current_num = current_num.append(digit.to_digit(10).unwrap());
                }
                ':' => {
                    game_id = current_num;
                    current_num = 0;
                }
                ';' => {
                    samples.push(current_sample);
                    current_sample = Sample::new();
                }
                'r' | 'g' | 'b' => {
                    if current_num == 0 {
                        continue;
                    };
                    match c {
                        'r' => current_sample.red = current_num,
                        'g' => current_sample.green = current_num,
                        'b' => current_sample.blue = current_num,
                        _ => {}
                    }
                    current_num = 0;
                }
                _ => {}
            }
        }

        samples.push(current_sample);

        Ok(Game {
            id: game_id,
            samples,
        })
    }

    fn is_possible(&self, max: &Sample) -> bool {
        for sample in self.samples.iter() {
            if !sample.is_possible(max) {
                return false;
            }
        }

        true
    }

    fn min_possible_sample(&self) -> Option<Sample> {
        let mut min = Sample::new();

        for next in self.samples.iter() {
            min.red = u32::max(min.red, next.red);
            min.green = u32::max(min.green, next.green);
            min.blue = u32::max(min.blue, next.blue);
        }

        Some(min)
    }
}

pub fn day2_part1(file: &mut io::BufReader<File>) -> u32 {
    let mut res: u32 = 0;
    let mut possible_games: Vec<u32> = Vec::new();

    for line in file.lines() {
        let game = Game::parse(line.unwrap()).unwrap();
        if game.is_possible(&Sample {
            red: 12,
            green: 13,
            blue: 14,
        }) {
            res += game.id;
            possible_games.push(game.id);
        }
    }

    println!("possible games: {:?}", possible_games);

    res
}

pub fn day2_part2(file: &mut io::BufReader<File>) -> u32 {
    let mut res: u32 = 0;

    for line in file.lines() {
        let game = Game::parse(line.unwrap()).unwrap();
        let min_sample = game.min_possible_sample().unwrap();
        res += min_sample.power();
    }

    res
}
