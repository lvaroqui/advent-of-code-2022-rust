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

    // The distress signal can only come from a place where two pair of diagonals
    // intersect like so:
    // #.#...#.#.
    // .#.#.#.#..
    // ..#.#.#...
    // ...#O#....
    // ..#.#.#...
    // .#.#.#.#..
    // #.#...#.#.
    // #: Line
    // .: Impossible space because part of a sensor "square"
    // O: Distress signal
    fn solve_2(&self, input: &str) -> DayResult {
        let sensors: Vec<_> = parse(input);

        // Find increasing and decreasing parallel lines that have y-intercepts
        // difference of 2
        let mut ord_increasing = vec![];
        let mut ord_decreasing = vec![];
        for s in &sensors {
            ord_increasing.push(s.pos.1 - (s.pos.0 + s.range));
            ord_increasing.push(s.pos.1 - (s.pos.0 - s.range));
            ord_decreasing.push(s.pos.1 + (s.pos.0 + s.range));
            ord_decreasing.push(s.pos.1 + (s.pos.0 - s.range));
        }

        let mut ord_possible_increasing = vec![];
        for (a, b) in ord_increasing.iter().sorted().tuples() {
            if b - a == 2 {
                ord_possible_increasing.push(b - 1);
            }
        }
        let mut ord_possible_decreasing = vec![];
        for (a, b) in ord_decreasing.iter().sorted().tuples() {
            if b - a == 2 {
                ord_possible_decreasing.push(b - 1);
            }
        }

        // Check intersection for valid distress signal location
        let bounds = 0..=4000000;
        for inc in &ord_possible_increasing {
            for dec in &ord_possible_decreasing {
                let x_inter = (dec - inc) / 2;
                let y_inter = x_inter + inc;
                if bounds.contains(&x_inter)
                    && bounds.contains(&y_inter)
                    && sensors
                        .iter()
                        .all(|s| manathan_dist((x_inter, y_inter), s.pos) > s.range)
                {
                    return DayResult::new(x_inter as i64 * 4_000_000 + y_inter as i64);
                }
            }
        }

        DayResult::default()
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
