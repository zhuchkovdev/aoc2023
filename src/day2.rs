use std::fs::File;
use std::io;
use std::io::BufRead;

enum Subset {
    Red(u32),
    Green(u32),
    Blue(u32),
}

trait AppendWith<T> {
    fn append(&self, value: T) -> Self;
}

impl AppendWith<u32> for u32 {
    fn append(&self, value: u32) -> Self {
        self * 10 + value
    }
}

struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

impl Game {
    fn parse(s: String) -> Result<Game, &'static str> {
        let mut game_id: u32 = 0;
        let mut subsets: Vec<Subset> = Vec::new();

        let mut current_num: u32 = 0;
        for c in s.strip_prefix("Game ").unwrap().chars() {
            match c {
                digit if c.is_digit(10) => {
                    current_num = current_num.append(digit.to_digit(10).unwrap());
                }
                ':' => {
                    game_id = current_num;
                    current_num = 0;
                }
                'r' | 'g' | 'b' => {
                    if current_num == 0 {continue};
                    match c {
                        'r' => subsets.push(Subset::Red(current_num)),
                        'g' => subsets.push(Subset::Green(current_num)),
                        'b' => subsets.push(Subset::Blue(current_num)),
                        _ => {},
                    }
                    current_num = 0;
                }
                _ => {}
            }
        }

        Ok(Game{ id: game_id, subsets })
    }

    fn is_possible(&self, max_rgb: [u32; 3]) -> bool {
        for subset in &self.subsets {
            let bigger = match subset {
                Subset::Red(val) => *val > max_rgb[0],
                Subset::Green(val) => *val > max_rgb[1],
                Subset::Blue(val) => *val > max_rgb[2],
            };

            if bigger {
                return false;
            }
        };

        return true;
    }
}

pub fn day2_part1(file: &mut io::BufReader<File>) -> u32 {
    let mut res: u32 = 0;

    for line in file.lines() {
        let game = Game::parse(line.unwrap()).unwrap();
        if game.is_possible([12, 13, 14]) {
            res += game.id
        }
    }

    res
}
