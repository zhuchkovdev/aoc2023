mod day4;

use std::fs::File;
use std::io::{self};
use std::path::Path;

fn main() {
    // match read_lines("./input/day1.txt") {
    //     Ok(mut buf) => println!("{}", day1::day1_part2(&mut buf)),
    //     Err(err) => println!("open file: {}", err),
    // }

    // match read_lines("./input/day2.txt") {
    //     Ok(mut buf) => println!("{}", day2::day2_part2(&mut buf)),
    //     Err(err) => println!("open file: {}", err),
    // }

    // match read_lines("./input/day3.txt") {
    //     Ok(mut buf) => println!("{:?}", day3::day3_part2(&mut buf)),
    //     Err(err) => println!("open file: {}", err),
    // }

    match read_lines("./input/day4.txt") {
        Ok(mut buf) => println!("{:?}", day4::day4_part2(&mut buf)),
        Err(err) => println!("open file: {}", err),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}
