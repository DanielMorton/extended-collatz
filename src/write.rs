use crate::collatz::Unsigned;
use csv::Writer;
use itertools::Itertools;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct Row {
    n: u64,
    cycle: String,
}

#[derive(Serialize)]
struct Cycle {
    n: String,
    count: usize,
    length: usize,
    cycle: String,
}

pub fn write_table(cycle_mins: &[Unsigned], n: &u64, a: &u64) -> () {
    if !Path::new("../tables").exists() {
        fs::create_dir("../tables").unwrap_or_else(|_| panic!("Cannot create directory 'tables'."));
    }
    let path = format!("tables/collatz{}.csv", a);
    let mut wtr = Writer::from_path(path).unwrap();
    for x in (1..=*n).step_by(2) {
        wtr.serialize(Row {
            n: x,
            cycle: match &cycle_mins[(x / 2) as usize] {
                Unsigned::BigInteger(b) => b.to_string(),
                Unsigned::U64(u) => u.to_string(),
                Unsigned::U128(u) => u.to_string(),
            },
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
    if !Path::new("../cycles").exists() {
        fs::create_dir("../cycles").unwrap_or_else(|_| panic!("Cannot create directory 'cycles'."));
    }
    let cycle_path = format!("cycles/cycle{}.csv", a);
    let mut wtr = Writer::from_path(cycle_path).unwrap();
    for c in cycles.keys().sorted() {
        let cycle_vec = cycles.get(&c).unwrap();
        let cycle_string = cycle_vec
            .iter()
            .map(|i| match i {
                Unsigned::BigInteger(b) => b.to_string(),
                Unsigned::U64(u) => u.to_string(),
                Unsigned::U128(u) => u.to_string(),
            })
            .join(" -> ");
        wtr.serialize(Cycle {
            n: match &c {
                Unsigned::BigInteger(b) => b.to_string(),
                Unsigned::U64(u) => u.to_string(),
                Unsigned::U128(u) => u.to_string(),
            },
            count: *cycle_counts.get(&c).unwrap(),
            length: cycle_vec.len(),
            cycle: cycle_string,
        })
        .unwrap();
    }
    wtr.flush().unwrap()
}
