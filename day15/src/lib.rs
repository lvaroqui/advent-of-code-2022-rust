use common::DayResult;
use itertools::Itertools;
use regex::Regex;

pub struct Solver;

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    range: i32,
}

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        let sensors: Vec<_> = parse(input);

        let y_line = 2000000;

        let itertools::MinMaxResult::MinMax(min_x, max_x) = sensors
            .iter()
            .flat_map(|s| [s.pos.0 - s.range, s.pos.0 + s.range])
            .minmax()
        else {
            unreachable!()
        };

        let mut sum = 0;
        for x in min_x..=max_x {
            for s in &sensors {
                let pos = (x, y_line);
                if manathan_dist(pos, s.pos) <= s.range && s.beacon != pos {
                    sum += 1;
                    break;
                }
            }
        }

        DayResult::new(sum)
    }
}

fn parse(input: &str) -> Vec<Sensor> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let sensor_pos = (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            );
            let beacon_pos = (
                cap[3].parse::<i32>().unwrap(),
                cap[4].parse::<i32>().unwrap(),
            );
            Sensor {
                pos: sensor_pos,
                beacon: beacon_pos,
                range: manathan_dist(sensor_pos, beacon_pos),
            }
        })
        .collect()
}

fn manathan_dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}
