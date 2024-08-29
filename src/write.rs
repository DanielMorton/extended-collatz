use crate::collatz::Unsigned;
use csv::Writer;
use itertools::Itertools;
use serde::Serialize;
use std::collections::{HashMap, VecDeque};

#[derive(Serialize)]
struct Row {
    n: u64,
    cycle: Unsigned,
}

#[derive(Serialize)]
struct Cycle {
    n: Unsigned,
    count: usize,
    length: usize,
    cycle: String,
}

pub fn write_table(cycle_mins: &[Unsigned], n: &u64, a: &u64) -> () {
    let path = format!("table/collatz{}.csv", a);
    let mut wtr = Writer::from_path(path).unwrap();
    for x in (1..=*n).step_by(2) {
        wtr.serialize(Row {
            n: x,
            cycle: cycle_mins[(x / 2) as usize],
        })
        .unwrap();
    }
    wtr.flush().unwrap();
}

pub fn write_cycle(
    cycles: &HashMap<Unsigned, Vec<Unsigned>>,
    cycle_counts: &HashMap<&Unsigned, usize>,
    a: &u64,
) -> () {
    let cycle_path = format!("cycle/cycle{}.csv", a);
    let mut wtr = Writer::from_path(cycle_path).unwrap();
    for &c in cycles.keys().sorted() {
        let cycle_vec = cycles.get(&c).unwrap();
        let cycle_string = cycle_vec.iter().map(|&i| i.to_string()).join(" -> ");
        wtr.serialize(Cycle {
            n: c,
            count: *cycle_counts.get(&c).unwrap(),
            length: cycle_vec.len(),
            cycle: cycle_string,
        })
        .unwrap();
    }
    wtr.flush().unwrap()
}
