use std::ops::BitAnd;

use common::DayResult;
use fixedbitset::FixedBitSet;
use itertools::Itertools;

fn to_bitset(s: &str) -> FixedBitSet {
    let mut res = FixedBitSet::with_capacity(53);
    for b in s.as_bytes().iter() {
        let b = match b {
            b'a'..=b'z' => 1 + b - b'a',
            b'A'..=b'Z' => 27 + b - b'A',
            _ => unreachable!(),
        };
        res.insert(b.into())
    }
    res
}

pub struct Day3;

impl common::DaySolver for Day3 {
    fn solve_1(&self, input: &str) -> DayResult {
        let res: usize = input
            .split('\n')
            .map(|l| {
                let (a, b) = l.split_at(l.len() / 2);
                let (a, ref b) = (to_bitset(a), to_bitset(b));
                a.bitand(b).ones().next().unwrap()
            })
            .sum();

        res.into()
    }

    fn solve_2(&self, input: &str) -> u64 {
        input
            .split('\n')
            .map(to_bitset)
            .tuples()
            .map(|(a, ref b, ref c)| (a.bitand(b).bitand(c)).ones().next().unwrap() as u64)
            .sum()
    }
}
