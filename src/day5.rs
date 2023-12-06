use std::cmp::{max, min};
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

// bruteforce approach
// TODO: maybe add multithreading if I wouldn't be able to fix overlapping ranges
pub fn solve_part2(file: &mut io::BufReader<File>) -> Result<i64, Box<dyn Error>> {
    let mut lines = file.lines();

    let seeds_ranges: Vec<i64> = lines
        .next()
        .ok_or("empty file")??
        .strip_prefix("seeds: ")
        .ok_or("bad seeds line")?
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    if seeds_ranges.len() % 2 != 0 {
        return Err("odd number of seeds")?;
    }

    let mut seeds: Vec<i64> = Vec::new();
    for seed in seeds_ranges.chunks(2) {
        let mut range_seeds: Vec<i64> = (seed[0]..seed[0] + seed[1]).into_iter().collect();
        seeds.append(&mut range_seeds);
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

trait RangeExt<T>
where
    Self: Sized,
{
    fn intersection(&self, other: &Self) -> Option<Self>;
    fn cutouts(&self, other: &Self) -> Vec<Self>;

    fn add(self, rhs: T) -> Self;
}

impl RangeExt<i64> for Range<i64> {
    fn intersection(&self, other: &Self) -> Option<Self> {
        let left = max(self.start, other.start);
        let right = min(self.end, other.end);

        if left < right {
            Some(left..right)
        } else {
            None
        }
    }

    fn cutouts(&self, other: &Self) -> Vec<Self> {
        if other.end <= self.start {
            vec![self.start..self.end]
        } else if other.start >= self.end {
            vec![self.start..self.end]
        } else if self.start < other.start && other.start < self.end && self.end <= other.end {
            vec![self.start..other.start]
        } else if self.start >= other.start && other.end > self.start && self.end > other.end {
            vec![other.end..self.end]
        } else if other.start > self.start && other.end < self.end {
            vec![self.start..other.start, other.end..self.end]
        } else {
            vec![]
        }
    }

    fn add(self, rhs: i64) -> Range<i64> {
        self.start + rhs..self.end + rhs
    }
}

//
pub fn solve_part2_not_working(file: &mut io::BufReader<File>) -> Result<i64, Box<dyn Error>> {
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

    let mut seeds_ranges: Vec<Range<i64>> = seeds
        .chunks(2)
        .map(|pair| pair[0]..pair[0] + pair[1])
        .collect();

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

    let mut new_seed_ranges: Vec<Range<i64>> = Vec::new();
    for map in maps {
        loop {
            match seeds_ranges.pop() {
                Some(seed_range) => {
                    for (i, map_range) in map.source_range.iter().enumerate() {
                        match seed_range.intersection(map_range) {
                            Some(inter) => {
                                seeds_ranges.append(&mut seed_range.cutouts(&inter));
                                new_seed_ranges.push(inter.add(map.offset[i]));
                            }
                            None => {
                                if i == map.source_range.len() - 1 {
                                    new_seed_ranges.push(seed_range.clone());
                                }
                            }
                        }
                    }
                }
                None => {
                    break;
                }
            };
        }
        seeds_ranges = new_seed_ranges;
        new_seed_ranges = Vec::new();
    }

    let mut min_loc = i64::MAX;
    for range in seeds_ranges {
        if range.start < min_loc {
            min_loc = range.start;
        }
    }

    Ok(min_loc)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        let range = 1..3;
        let res = range.add(5);
        assert_eq!(res, 6..8);
    }

    #[test]
    fn intersection() {
        let range = 3..8;

        let inter = range.intersection(&(3_i64..8_i64));
        assert_eq!(inter, Some(3..8));

        let inter = range.intersection(&(0_i64..2_i64));
        assert_eq!(inter, None);

        let inter = range.intersection(&(0_i64..3_i64));
        assert_eq!(inter, None);

        let inter = range.intersection(&(0_i64..4_i64));
        assert_eq!(inter, Some(3..4));

        let inter = range.intersection(&(3_i64..6_i64));
        assert_eq!(inter, Some(3..6));

        let inter = range.intersection(&(5_i64..6_i64));
        assert_eq!(inter, Some(5..6));

        let inter = range.intersection(&(5_i64..8_i64));
        assert_eq!(inter, Some(5..8));

        let inter = range.intersection(&(5_i64..10_i64));
        assert_eq!(inter, Some(5..8));

        let inter = range.intersection(&(8_i64..10_i64));
        assert_eq!(inter, None);
    }

    #[test]
    fn cutouts() {
        let range = 3..8;

        let inter = range.cutouts(&(3_i64..8_i64));
        assert_eq!(inter, vec![]);

        let inter = range.cutouts(&(0_i64..2_i64));
        assert_eq!(inter, vec![3..8]);

        let inter = range.cutouts(&(0_i64..3_i64));
        assert_eq!(inter, vec![3..8]);

        let inter = range.cutouts(&(0_i64..4_i64));
        assert_eq!(inter, vec![4..8]);

        let inter = range.cutouts(&(3_i64..6_i64));
        assert_eq!(inter, vec![6..8]);

        let inter = range.cutouts(&(5_i64..6_i64));
        assert_eq!(inter, vec![3..5, 6..8]);

        let inter = range.cutouts(&(5_i64..8_i64));
        assert_eq!(inter, vec![3..5]);

        let inter = range.cutouts(&(5_i64..10_i64));
        assert_eq!(inter, vec![3..5]);

        let inter = range.cutouts(&(8_i64..10_i64));
        assert_eq!(inter, vec![3..8]);

        let inter = range.cutouts(&(9_i64..10_i64));
        assert_eq!(inter, vec![3..8]);
    }
}
