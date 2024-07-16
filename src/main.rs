#![allow(warnings)]
use crate::collatz::extended_collatz;
use csv::Writer;
use rayon::prelude::{ParallelIterator, IntoParallelIterator};
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::time::Instant;
use itertools::Itertools;

mod collatz;

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

fn print_hms(start: &Instant) {
    let millis = start.elapsed().as_millis();
    let seconds = millis / 1000;
    let (hour, minute, second) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    println!("{:02}:{:02}:{:02}.{}", hour, minute, second, millis % 1000)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<u64>().unwrap();
    let a_start = args[2].parse::<u64>().unwrap();
    let a_end = args[3].parse::<u64>().unwrap();
    let start = Instant::now();
    (a_start..=a_end)
        .into_par_iter()
        .filter(|&a| a & 1 == 1)
        .for_each(|a| {
            let mut p = 1;
            while p < a {
                p <<= 1;
            }
            p >>= 1;
            let mut cycle_mins = HashMap::new();
            let mut cycle_map = HashMap::new();
            let mut cycles = HashMap::new();
            (1..=n).step_by(2).for_each(|x| {
                extended_collatz(x, a, p, &mut cycle_mins, &mut cycle_map, &mut cycles);
            });
            /*let path = format!("collatz{}.csv", a);
            et mut wtr = Writer::from_path(path).unwrap();
            for x in (1..=n).step_by(2) {
                wtr.serialize(Row {
                    n: x,
                    cycle: *cycle_map.get(&x).unwrap(),
                })
                .unwrap();
            }
            wtr.flush().unwrap();
             */
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
                    count: *cycle_mins.get(&c).unwrap(),
                    length: cycle_vec.len(),
                    cycle: cycle_string,
                })
                .unwrap();
            }
            wtr.flush().unwrap()
        });
    print_hms(&start)
}
