use common::DayResult;
use itertools::Itertools;

pub struct Day3;

impl common::DaySolver for Day3 {
    fn solve_1(&self, input: &str) -> DayResult {
        input
            .split('\n')
            .map(|l| {
                let (a, b) = l.split_at(l.len() / 2);
                let (a, b) = (to_bitset(a), to_bitset(b));
                Ones::new(a & b).next().unwrap()
            })
            .sum::<u64>()
            .into()
    }

    fn solve_2(&self, input: &str) -> u64 {
        input
            .split('\n')
            .map(to_bitset)
            .tuples()
            .map(|(a, ref b, ref c)| Ones::new(a & b & c).next().unwrap())
            .sum()
    }
}

fn to_bitset(s: &str) -> u64 {
    let mut res = 0;
    for b in s.as_bytes().iter() {
        let b = match b {
            b'a'..=b'z' => 1 + b - b'a',
            b'A'..=b'Z' => 27 + b - b'A',
            _ => unreachable!(),
        };
        res |= 1 << b;
    }
    res
}

struct Ones {
    inner: u64,
    index: u8,
}

impl Ones {
    fn new(inner: u64) -> Self {
        Self { inner, index: 0 }
    }
}

impl Iterator for Ones {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.inner != 0 {
            let v = self.inner & 1;
            self.index += 1;
            self.inner >>= 1;
            if v == 1 {
                return Some((self.index - 1).into());
            }
        }
        None
    }
}
