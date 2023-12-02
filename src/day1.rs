use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn day1_part1(file: &mut io::BufReader<File>) -> u32 {
    let mut sum: u32 = 0;

    for line in file.lines() {
        let mut first: u32 = 0;
        let mut last: u32 = 0;

        for c in line.unwrap().chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();
                if first == 0 {
                    first = digit;
                }
                last = digit;
            }
        }

        sum += first * 10 + last;
    }

    sum
}

struct Digits {
    strings: [&'static str; 9],
    counters: [u8; 9],
}

impl Digits {
    fn new() -> Digits {
        Digits {
            strings: [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ],
            counters: [0; 9],
        }
    }

    fn advance_counters(&mut self, c: char) -> Option<u8> {
        let mut res: Option<u8> = None;

        for i in 0..self.counters.len() {
            let digit_chars: Vec<_> = self.strings[i].chars().collect();
            if c == digit_chars[self.counters[i] as usize] {
                self.counters[i] += 1;
                if self.counters[i] == digit_chars.len() as u8 {
                    self.counters[i] = 0;
                    res = Some((i + 1) as u8)
                }
            } else {
                self.counters[i] = if c == digit_chars[0] { 1 } else { 0 };
            }
        }

        res
    }
}

pub fn day1_part2(file: &mut io::BufReader<File>) -> u32 {
    let mut sum: u32 = 0;

    for line in file.lines() {
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        let mut digit_tracker = Digits::new();

        for c in line.unwrap().chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();
                if first == 0 {
                    first = digit;
                }
                last = digit;
            } else if c.is_alphabetic() {
                if let Some(digit) = digit_tracker.advance_counters(c) {
                    if first == 0 {
                        first = digit as u32;
                    }
                    last = digit as u32;
                }
            }
        }

        sum += first * 10 + last;
    }

    sum
}
