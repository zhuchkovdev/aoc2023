use std::cmp::min;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Range;

#[derive(Debug)]
struct Map {
    name: String,
    source_range: Vec<Range<i64>>,
    offset: Vec<i64>,
}

pub fn solve_part1(file: &mut io::BufReader<File>) -> Result<i64, Box<dyn Error>> {
    let mut lines = file.lines();

    let seeds: Vec<i64> = lines
        .next()
        .ok_or("empty file")??
        .strip_prefix("seeds: ")
        .ok_or("bad seeds line")?
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    if seeds.len() % 2 != 0 {
        return Err("odd number of seeds")?;
    }

    let mut maps: Vec<Map> = Vec::with_capacity(7);
    let mut current_map: Option<Map> = None;
    for line in lines {
        let line = line?;

        if line.len() == 0 {
            continue;
        }

        if line.ends_with(':') {
            if current_map.is_some() {
                maps.push(current_map.unwrap());
            }

            current_map = Some(Map {
                name: line
                    .strip_suffix(" map:")
                    .ok_or("bad map name")?
                    .to_string(),
                source_range: Vec::new(),
                offset: Vec::new(),
            });
            continue;
        }

        let nums: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();

        if nums.len() != 3 {
            return Err("bad nums line")?;
        }

        let current_map = current_map.as_mut().ok_or("missing map entry")?;
        current_map.source_range.push(nums[1]..nums[1] + nums[2]);
        current_map.offset.push(nums[0] - nums[1]);
    }

    if current_map.is_some() {
        maps.push(current_map.unwrap());
    }

    // get seeds
    let mut min_location = i64::MAX;
    for seed in seeds {
        let mut source = seed;
        '_maps: for map in &maps {
            for (i, range) in map.source_range.iter().enumerate() {
                if range.contains(&source) {
                    source = source + map.offset[i];
                    continue '_maps;
                }
            }
        }
        min_location = min(min_location, source);
    }

    Ok(min_location)
}
