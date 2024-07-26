use csv::Writer;
use itertools::Itertools;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Row {
    n: u64,
    cycle: u64,
}

#[derive(Serialize)]
struct Cycle {
    n: u64,
    count: u64,
    length: usize,
    cycle: String,
}

pub fn write_table(cycle_mins: &[u64], n: &u64, a: &u64) -> () {
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
    cycles: &HashMap<u64, Vec<u64>>,
    cycle_counts: &HashMap<u64, u64>,
    a: &u64,
) -> () {
    let cycle_path = format!("cycle/cycle{}.csv", a);
    let mut wtr = Writer::from_path(cycle_path).unwrap();
    for &c in cycles.keys().sorted() {
        let cycle_vec = cycles.get(&c).unwrap();
        let cycle_string = cycle_vec
            .iter()
            .map(|&i| i.to_string() + " ")
            .collect::<String>()
            .strip_suffix(' ')
            .unwrap()
            .to_owned();
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
