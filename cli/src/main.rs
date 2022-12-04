use std::{
    fmt::Display,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
    time::{Duration, Instant},
};

use anyhow::Context;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use common::DaySolver;
use reqwest::blocking::Client;

fn main() -> anyhow::Result<()> {
    let day: u8 = std::env::args()
        .nth(1)
        .with_context(|| "Please provide a day number")?
        .parse()?;

    let solver: &dyn DaySolver = match day {
        3 => &day03::Day3,
        4 => &day04::Day4,
        _ => anyhow::bail!("Day {} not implemented!", day),
    };

    let mut path = PathBuf::from_str("inputs")?;
    std::fs::create_dir_all(&path)?;
    path.push(day.to_string());
    if !path.exists() {
        let mut f = std::fs::File::create(&path)?;

        let client = Client::new();
        let session_key = std::fs::read_to_string("session-key")?;
        let mut req = client
            .get(format!("https://adventofcode.com/2022/day/{}/input", day))
            .header("Cookie", format!("session={}", session_key))
            .send()?;
        let mut buf = [0; 4096];
        while let Ok(w) = req.read(&mut buf) {
            if w == 0 {
                break;
            }
            f.write_all(&buf[0..w])?;
        }
    }

    let input = std::fs::read_to_string(path)?;
    let input = input.trim();

    let (result, first_stats) = instrument(|| solver.solve_1(input));
    let first = result.first();

    let (second, second_stats) = if let Some(second) = result.second() {
        (second, None)
    } else {
        let (r, s) = instrument(|| solver.solve_2(input));
        (r, Some(s))
    };

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Day", "First Part", "Second Part"])
        .add_row(vec![
            day.to_string(),
            format!("{} ({})", first, first_stats),
            format!(
                "{} ({})",
                second,
                if let Some(second_stats) = second_stats {
                    second_stats.to_string()
                } else {
                    "Calculated during first".to_string()
                }
            ),
        ]);

    println!("{table}");

    Ok(())
}

struct Stats {
    duration: Duration,
}

fn instrument<T>(f: impl FnOnce() -> T) -> (T, Stats) {
    let n = Instant::now();
    let res = f();
    let duration = n.elapsed();

    (res, Stats { duration })
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.duration)
    }
}
