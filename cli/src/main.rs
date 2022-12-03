use std::{
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};

use common::DaySolver;
use reqwest::blocking::Client;

fn main() {
    let day: u8 = std::env::args().nth(1).unwrap().parse().unwrap();

    let solver: &dyn DaySolver = match day {
        3 => &day03::Day3,
        _ => unimplemented!(),
    };

    let mut path = PathBuf::from_str("inputs").unwrap();
    std::fs::create_dir_all(&path).unwrap();
    path.push(day.to_string());
    if !path.exists() {
        let mut f = std::fs::File::create(&path).unwrap();

        let client = Client::new();
        let session_key = std::fs::read_to_string("session-key").unwrap();
        let mut req = client
            .get(format!("https://adventofcode.com/2022/day/{}/input", day))
            .header("Cookie", format!("session={}", session_key))
            .send()
            .unwrap();
        let mut buf = [0; 4096];
        while let Ok(w) = req.read(&mut buf) {
            if w == 0 {
                break;
            }
            f.write_all(&buf[0..w]).unwrap();
        }
    }

    let input = std::fs::read_to_string(path).unwrap();
    let input = input.trim();

    let result = solver.solve_1(input);
    println!("{}", result.first());

    if let Some(second) = result.second() {
        println!("{}", second)
    } else {
        println!("{}", solver.solve_2(input));
    }
}
