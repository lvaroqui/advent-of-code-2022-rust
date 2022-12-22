use std::{
    collections::{HashMap, HashSet},
    str::from_utf8,
};

use common::DayResult;
use petgraph::{graph::NodeIndex, Direction::Outgoing, Graph};
use regex::Regex;

pub struct Solver;

#[derive(Debug)]
struct Valve {
    id: u16,
    flow_rate: i32,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State1 {
    activated: u64,
    score: i32,
    cur_pos: NodeIndex,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State1Cache {
    activated: u64,
    cur_pos: NodeIndex,
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State2 {
    activated: u64,
    score: i32,
    cur_pos: NodeIndex,
    cur_pos_elephant: NodeIndex,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State2Cache {
    activated: u64,
    cur_pos: NodeIndex,
    cur_pos_elephant: NodeIndex,
}

impl State2 {
    fn normalize(&mut self) {
        if self.cur_pos > self.cur_pos_elephant {
            std::mem::swap(&mut self.cur_pos, &mut self.cur_pos_elephant);
        }
    }
}

impl common::DualDaySolver for Solver {
    fn solve_1(&self, input: &str) -> DayResult {
        let graph = parse(input);

        let mut current_states = vec![State1 {
            activated: 0,
            score: 0,
            cur_pos: NodeIndex::new(
                graph
                    .raw_nodes()
                    .iter()
                    .enumerate()
                    .find(|(_i, n)| n.weight.id == name_to_u16("AA"))
                    .map(|(i, _)| i)
                    .unwrap(),
            ),
        }];
        let mut next_states = vec![];

        let mut prev_cache = HashSet::new();

        let mut max = 0;

        for remaining_time in (1..=29).into_iter().rev() {
            let mut new_cache = HashSet::new();
            for s in current_states {
                for n in graph.neighbors_directed(s.cur_pos, Outgoing) {
                    let mut new_state = s.clone();
                    new_state.cur_pos = n;
                    push_state1(new_state, &mut next_states, &prev_cache, &mut new_cache);
                }
                if !get_bit(s.activated, s.cur_pos.index()) && graph[s.cur_pos].flow_rate != 0 {
                    let mut new_state = s.clone();
                    set_bit(&mut new_state.activated, new_state.cur_pos.index());
                    new_state.score += remaining_time * graph[new_state.cur_pos].flow_rate;
                    push_state1(new_state, &mut next_states, &prev_cache, &mut new_cache);
                }
            }
            current_states = next_states;
            next_states = vec![];
            prev_cache.extend(new_cache);
            max = max.max(current_states.iter().map(|s| s.score).max().unwrap_or(0));
        }

        DayResult::new(max)
    }

    fn solve_2(&self, input: &str) -> DayResult {
        let graph = parse(input);
        let start_node = NodeIndex::new(
            graph
                .raw_nodes()
                .iter()
                .enumerate()
                .find(|(_i, n)| n.weight.id == name_to_u16("AA"))
                .map(|(i, _)| i)
                .unwrap(),
        );
        let mut current_states = vec![State2 {
            activated: 0,
            score: 0,
            cur_pos: start_node,
            cur_pos_elephant: start_node,
        }];
        let mut next_states = vec![];

        let mut prev_cache = HashSet::new();
        let mut max = 0;
        for remaining_time in (1..=25).into_iter().rev() {
            println!("{} {}", remaining_time, current_states.len());
            let mut new_cache = HashSet::new();
            for s in current_states {
                let me_dest: Vec<_> = graph.neighbors_directed(s.cur_pos, Outgoing).collect();
                let elephant_dest: Vec<_> = graph
                    .neighbors_directed(s.cur_pos_elephant, Outgoing)
                    .collect();

                for m in &me_dest {
                    for e in &elephant_dest {
                        let mut new_state = s.clone();
                        new_state.cur_pos = *m;
                        new_state.cur_pos_elephant = *e;
                        push_state2(new_state, &mut next_states, &prev_cache, &mut new_cache);
                    }
                }
                if !get_bit(s.activated, s.cur_pos.index()) && graph[s.cur_pos].flow_rate != 0 {
                    for e in &elephant_dest {
                        let mut new_state = s.clone();
                        set_bit(&mut new_state.activated, new_state.cur_pos.index());
                        new_state.score += remaining_time * graph[new_state.cur_pos].flow_rate;
                        new_state.cur_pos_elephant = *e;
                        max = max.max(new_state.score);
                        push_state2(new_state, &mut next_states, &prev_cache, &mut new_cache);
                    }
                }
                if !get_bit(s.activated, s.cur_pos_elephant.index())
                    && graph[s.cur_pos_elephant].flow_rate != 0
                {
                    for m in &me_dest {
                        let mut new_state = s.clone();
                        set_bit(&mut new_state.activated, new_state.cur_pos_elephant.index());
                        new_state.score +=
                            remaining_time * graph[new_state.cur_pos_elephant].flow_rate;
                        new_state.cur_pos = *m;
                        max = max.max(new_state.score);
                        push_state2(new_state, &mut next_states, &prev_cache, &mut new_cache);
                    }
                }
                if (!get_bit(s.activated, s.cur_pos.index()) && graph[s.cur_pos].flow_rate != 0)
                    && (!get_bit(s.activated, s.cur_pos_elephant.index())
                        && graph[s.cur_pos_elephant].flow_rate != 0)
                    && (s.cur_pos != s.cur_pos_elephant)
                {
                    let mut new_state = s.clone();
                    set_bit(&mut new_state.activated, new_state.cur_pos.index());
                    new_state.score += remaining_time * graph[new_state.cur_pos].flow_rate;
                    set_bit(&mut new_state.activated, new_state.cur_pos_elephant.index());
                    new_state.score += remaining_time * graph[new_state.cur_pos_elephant].flow_rate;
                    max = max.max(new_state.score);
                    push_state2(new_state, &mut next_states, &prev_cache, &mut new_cache);
                }
            }
            next_states.sort_unstable();
            next_states.reverse();

            next_states.dedup_by(|s1, s2| {
                (s1.activated, s1.cur_pos, s1.cur_pos_elephant)
                    == (s2.activated, s2.cur_pos, s2.cur_pos_elephant)
            });
            current_states = next_states;
            next_states = vec![];
            prev_cache.extend(new_cache);
        }

        DayResult::new(max)
    }
}

fn push_state1(
    state: State1,
    dest: &mut Vec<State1>,
    prev_cache: &HashSet<State1Cache>,
    new_cache: &mut HashSet<State1Cache>,
) {
    let state1_cache = State1Cache {
        activated: state.activated,
        cur_pos: state.cur_pos,
    };
    if !prev_cache.contains(&state1_cache) {
        new_cache.insert(state1_cache);
        dest.push(state)
    }
}

fn push_state2(
    mut state: State2,
    dest: &mut Vec<State2>,
    prev_cache: &HashSet<State2Cache>,
    new_cache: &mut HashSet<State2Cache>,
) {
    state.normalize();
    let state2_cache = State2Cache {
        activated: state.activated,
        cur_pos: state.cur_pos,
        cur_pos_elephant: state.cur_pos_elephant,
    };
    if !prev_cache.contains(&state2_cache) {
        dest.push(state);
        new_cache.insert(state2_cache);
    }
}

fn get_bit(val: u64, bit: usize) -> bool {
    (val >> bit) & 1 == 1
}

fn set_bit(val: &mut u64, bit: usize) {
    let mask = 1 << bit;
    *val |= mask;
}

fn parse(input: &str) -> Graph<Valve, ()> {
    let re =
        Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();

    let mut graph = Graph::new();
    let mut m = HashMap::new();
    for cap in re.captures_iter(input) {
        let name = cap[1].to_string();
        let n = graph.add_node(Valve {
            id: name_to_u16(&name),
            flow_rate: cap[2].parse().unwrap(),
        });
        let edges = cap[3]
            .split(',')
            .map(|s| s.trim())
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        m.insert(name, (n, edges));
    }

    for (from, edges) in m.values() {
        for e in edges {
            let to = m.get(e).unwrap().0;
            graph.add_edge(*from, to, ());
        }
    }

    graph
}

fn name_to_u16(name: &str) -> u16 {
    assert!(name.len() == 2);
    u16::from_le_bytes(name.as_bytes().try_into().unwrap())
}

fn u16_to_name(id: u16) -> String {
    from_utf8(&id.to_le_bytes()).unwrap().to_owned()
}
