use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

pub fn day4_part1(file: &mut io::BufReader<File>) -> Result<u32, Box<dyn std::error::Error>> {
    let re = Regex::new(r"Card +[0-9]+:(?<winning>[ 0-9]+)\|(?<got>[ 0-9]+)")?;
    let mut sum: u32 = 0;

    for line in file.lines() {
        for (_, [winning, got]) in re.captures_iter(line?.as_str()).map(|c| c.extract()) {
            let winning: HashSet<u32> = winning
                .split_whitespace()
                .map(|s| u32::from_str(s).unwrap())
                .collect();
            let got: HashSet<u32> = got
                .split_whitespace()
                .map(|s| u32::from_str(s).unwrap())
                .collect();

            let matches = winning.intersection(&got).count() as u32;

            if matches > 0 {
                sum += 2_u32.pow(matches - 1);
            }
        }
    }

    Ok(sum)
}

pub fn day4_part2(file: &mut io::BufReader<File>) -> Result<u32, Box<dyn std::error::Error>> {
    let re = Regex::new(r"Card +(?<index>[0-9]+):(?<winning>[ 0-9]+)\|(?<got>[ 0-9]+)")?;
    let mut bonus_cards: HashMap<u32, u32> = HashMap::new();
    let mut cards_amount = 0;

    for line in file.lines() {
        for (_, [index, winning, got]) in re.captures_iter(line?.as_str()).map(|c| c.extract()) {
            let card_index = u32::from_str(index)?;
            let winning: HashSet<u32> = winning
                .split_whitespace()
                .map(|s| u32::from_str(s).unwrap())
                .collect();
            let got: HashSet<u32> = got
                .split_whitespace()
                .map(|s| u32::from_str(s).unwrap())
                .collect();

            let matches = winning.intersection(&got).count() as u32;
            let bonus = bonus_cards.get(&card_index).cloned().unwrap_or(1);
            for i in card_index + 1..=card_index + matches {
                *bonus_cards.entry(i).or_insert(1) += bonus;
            }
        }
        cards_amount += 1
    }

    Ok(bonus_cards.values().fold(0, |acc, e| acc + e - 1) + cards_amount)
}
