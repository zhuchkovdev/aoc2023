use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Map;

trait AppendWith<T> {
    fn append(&self, value: T) -> Self;
}

impl AppendWith<u32> for u32 {
    fn append(&self, value: u32) -> Self {
        self * 10 + value
    }
}

pub fn day3_part1(file: &mut io::BufReader<File>) -> Result<u32, &'static str> {
    let mut sum: u32 = 0;

    let mut lines = file.lines();
    let mut cur_line: Vec<u8>;
    let mut prev_line: Option<Vec<u8>> = None;
    let mut next_line: Option<Vec<u8>>;

    match lines.next() {
        Some(line) => cur_line = line.unwrap().into_bytes(),
        None => return Err("empty file"),
    };

    for line in lines {
        next_line = Some(line.unwrap().into_bytes());

        sum += get_row_sum(&cur_line, &prev_line, &next_line);

        prev_line = Some(cur_line);
        cur_line = next_line.unwrap();
    }

    next_line = None;
    sum += get_row_sum(&cur_line, &prev_line, &next_line);

    Ok(sum)
}

fn get_row_sum(cur: &Vec<u8>, prev: &Option<Vec<u8>>, next: &Option<Vec<u8>>) -> u32 {
    let mut sum: u32 = 0;

    let mut cur_num: u32 = 0;
    let mut num_start: usize = 0;
    for i in 0..cur.len() {
        if cur[i].is_ascii_digit() {
            if cur_num == 0 {
                num_start = i;
            }
            cur_num = cur_num.append((cur[i] - b'0') as u32);
            continue;
        }

        if cur_num == 0 {
            continue;
        }

        if is_part_number(num_start, i - 1, cur, prev, next) {
            sum += cur_num;
        } else {
        }

        cur_num = 0;
    }

    if cur_num != 0 {
        if is_part_number(num_start, cur.len() - 1, cur, prev, next) {
            sum += cur_num;
        }
    }

    sum
}

fn is_part_number(
    start: usize,
    end: usize,
    cur: &Vec<u8>,
    prev: &Option<Vec<u8>>,
    next: &Option<Vec<u8>>,
) -> bool {
    #[rustfmt::skip]
    let check = |ch: u8| -> bool { 
        !ch.is_ascii_digit() && ch != b'.' 
    };

    if let Some(prev) = prev {
        for i in start..=end {
            if check(prev[i]) {
                return true;
            }
        }

        if start >= 1 && check(prev[start - 1]) {
            return true;
        }

        if end < prev.len() - 1 && check(prev[end + 1]) {
            return true;
        }
    }

    if let Some(next) = next {
        for i in start..=end {
            if check(next[i]) {
                return true;
            }
        }

        if start >= 1 && check(next[start - 1]) {
            return true;
        }

        if end < next.len() - 1 && check(next[end + 1]) {
            return true;
        }
    }

    if start >= 1 && check(cur[start - 1]) {
        return true;
    }

    if end < cur.len() - 1 && check(cur[end + 1]) {
        return true;
    }

    false
}

struct Line {
    index: usize,
    content: Vec<u8>,
}

type Position = (usize, usize);

struct AdjacentNumber {
    gear_position: Position,
    num: u32,
}

pub fn day3_part2(file: &mut io::BufReader<File>) -> Result<u32, &'static str> {
    let mut lines = file.lines();
    let mut cur_line: Line;
    let mut prev_line: Option<Line> = None;
    let mut next_line: Option<Line>;

    let mut adjacent_nums: HashMap<Position, Vec<AdjacentNumber>> = HashMap::new();
    match lines.next() {
        Some(line) => {
            cur_line = Line {
                index: 0,
                content: line.unwrap().into_bytes(),
            }
        }
        None => return Err("empty file"),
    };

    for (i, line) in lines.enumerate() {
        next_line = Some(Line {
            content: line.unwrap().into_bytes(),
            index: i,
        });

        let row_adjacent_nums = get_adjacent_nums(&cur_line, &prev_line, &next_line);
        for num in row_adjacent_nums {
            if let Some(nums) = adjacent_nums.get_mut(&num.gear_position) {
                nums.push(num)
            } else {
                adjacent_nums.insert(num.gear_position, vec![num]);
            }
        }

        prev_line = Some(cur_line);
        cur_line = next_line.unwrap();
    }

    next_line = None;
    let row_adjacent_nums = get_adjacent_nums(&cur_line, &prev_line, &next_line);
    for num in row_adjacent_nums {
        if let Some(nums) = adjacent_nums.get_mut(&num.gear_position) {
            nums.push(num)
        } else {
            adjacent_nums.insert(num.gear_position, vec![num]);
        }
    }

    let mut sum: u32 = 0;
    for (_, nums) in adjacent_nums {
        if nums.len() == 2 {
            sum += nums.get(0).unwrap().num * nums.get(1).unwrap().num;
        }
    }

    Ok(sum)
}

fn get_adjacent_nums(
    cur: &Line,
    prev: &Option<Line>,
    next: &Option<Line>,
) -> Vec<(AdjacentNumber)> {
    let mut res: Vec<AdjacentNumber> = Vec::new();

    let mut cur_num: u32 = 0;
    let mut num_start: usize = 0;
    for i in 0..cur.content.len() {
        if cur.content[i].is_ascii_digit() {
            if cur_num == 0 {
                num_start = i;
            }
            cur_num = cur_num.append((cur.content[i] - b'0') as u32);
            continue;
        }

        if cur_num == 0 {
            continue;
        }

        get_adjacent_gear_positions(num_start, i - 1, cur, prev, next)
            .iter()
            .cloned()
            .for_each(|x| {
                res.push(AdjacentNumber {
                    gear_position: x,
                    num: cur_num,
                });
            });

        cur_num = 0;
    }

    if cur_num != 0 {
        get_adjacent_gear_positions(num_start, cur.content.len() - 1, cur, prev, next)
            .iter()
            .cloned()
            .for_each(|x| {
                res.push(AdjacentNumber {
                    gear_position: x,
                    num: cur_num,
                });
            });
    }

    res
}

fn get_adjacent_gear_positions(
    start: usize,
    end: usize,
    cur: &Line,
    prev: &Option<Line>,
    next: &Option<Line>,
) -> Vec<Position> {
    let mut res: Vec<Position> = Vec::new();

    let check = |ch: u8| -> bool { ch == b'*' };

    if let Some(prev) = prev {
        for i in start..=end {
            if check(prev.content[i]) {
                res.push((prev.index, i))
            }
        }

        if start >= 1 && check(prev.content[start - 1]) {
            res.push((prev.index, start - 1))
        }

        if end < prev.content.len() - 1 && check(prev.content[end + 1]) {
            res.push((prev.index, end + 1))
        }
    }

    if let Some(next) = next {
        for i in start..=end {
            if check(next.content[i]) {
                res.push((next.index, i))
            }
        }

        if start >= 1 && check(next.content[start - 1]) {
            res.push((next.index, start - 1))
        }

        if end < next.content.len() - 1 && check(next.content[end + 1]) {
            res.push((next.index, end + 1))
        }
    }

    if start >= 1 && check(cur.content[start - 1]) {
        res.push((cur.index, start - 1))
    }

    if end < cur.content.len() - 1 && check(cur.content[end + 1]) {
        res.push((cur.index, end + 1))
    }

    res
}
