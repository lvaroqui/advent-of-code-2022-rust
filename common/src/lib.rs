pub struct DayResult(u64, Option<u64>);

pub trait DaySolver {
    fn solve_1(&self, input: &str) -> DayResult;

    #[allow(unused_variables)]
    fn solve_2(&self, input: &str) -> u64 {
        unimplemented!()
    }
}

impl<T> From<T> for DayResult
where
    T: TryInto<u64>,
    <T as std::convert::TryInto<u64>>::Error: std::fmt::Debug,
{
    fn from(v: T) -> Self {
        DayResult(v.try_into().unwrap(), None)
    }
}

impl DayResult {
    pub fn first(&self) -> u64 {
        self.0
    }

    pub fn second(&self) -> Option<u64> {
        self.1
    }

    pub fn solve_both<T1, T2>(first: T1, second: T2) -> Self
    where
        T1: TryInto<u64>,
        <T1 as std::convert::TryInto<u64>>::Error: std::fmt::Debug,
        T2: TryInto<u64>,
        <T2 as std::convert::TryInto<u64>>::Error: std::fmt::Debug,
    {
        Self(first.try_into().unwrap(), Some(second.try_into().unwrap()))
    }
}
