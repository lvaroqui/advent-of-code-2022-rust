use common::DayResult;

pub struct Solver;

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        solve(input, |stacks, from, to, amount| {
            for _ in 0..amount {
                let c = stacks[from].pop().unwrap();
                stacks[to].push(c);
            }
        })
    }

    fn solve_2(&self, input: &str) -> DayResult {
        solve(input, |stacks, from, to, amount| {
            let i = stacks[from].len();
            let tmp = stacks[from].split_off(i - amount);
            stacks[to].extend_from_slice(&tmp);
        })
    }
}

fn solve(input: &str, stack_func: impl Fn(&mut Vec<Vec<char>>, usize, usize, usize)) -> DayResult {
    let (layout, moves) = {
        let mut it = input.split("\n\n");
        (it.next().unwrap(), it.next().unwrap())
    };

    let layout: Vec<_> = layout.split('\n').collect();
    let stack_count = layout
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut stacks = vec![vec![]; stack_count];

    for &l in layout.iter().rev().skip(1) {
        let mut it = l.chars();
        let mut i = 0;
        while let Some(c) = it.nth(1) {
            if c != ' ' {
                stacks[i].push(c);
            }
            i += 1;
            it.nth(1);
        }
    }

    for (amount, from, to) in moves.split('\n').map(|l| {
        let mut it = l.split_whitespace();
        (
            it.nth(1).unwrap().parse::<usize>().unwrap(),
            it.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            it.nth(1).unwrap().parse::<usize>().unwrap() - 1,
        )
    }) {
        stack_func(&mut stacks, from, to, amount);
    }

    let res: String = stacks.iter().map(|s| s.last().unwrap()).collect();
    DayResult::new(res)
}
