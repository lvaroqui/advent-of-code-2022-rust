use std::ops::RangeInclusive;

use common::DayResult;

pub struct Day4;

impl common::DaySolver for Day4 {
    fn solve_1(&self, input: &str) -> DayResult {
        get_ranges(input)
            .filter(|(a, b)| {
                (a.contains(b.start()) && a.contains(b.end()))
                    || (b.contains(a.start()) && b.contains(a.end()))
            })
            .count()
            .into()
    }

    fn solve_2(&self, input: &str) -> u64 {
        get_ranges(input)
            .filter(|(a, b)| {
                a.contains(b.start())
                    || a.contains(b.end())
                    || b.contains(a.start())
                    || b.contains(a.end())
            })
            .count() as u64
    }
}

fn get_ranges(
    input: &str,
) -> impl Iterator<Item = (RangeInclusive<u64>, RangeInclusive<u64>)> + '_ {
    input.split('\n').map(|l| {
        let mut it = l.split(',');
        (to_range(it.next().unwrap()), to_range(it.next().unwrap()))
    })
}

fn to_range(s: &str) -> RangeInclusive<u64> {
    let mut it = s.split('-');
    it.next().unwrap().parse().unwrap()..=it.next().unwrap().parse().unwrap()
}
