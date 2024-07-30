#![allow(warnings)]
use crate::collatz::extended_collatz;
use crate::write::{write_cycle, write_table};
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;
use std::time::Instant;

mod collatz;
mod parse;
mod write;
mod big_collatz;

fn print_hms(start: &Instant) {
    let millis = start.elapsed().as_millis();
    let seconds = millis / 1000;
    let (hour, minute, second) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    println!("{:02}:{:02}:{:02}.{}", hour, minute, second, millis % 1000)
}

fn main() {
    let matches = parse::parse();
    let n = match matches.get_one::<u64>("n") {
        Some(&n) => n,
        None => panic!("Number of iterations not provided."),
    };
    let a_start = match matches.get_one::<u64>("start") {
        Some(&s) => s,
        None => panic!("No initial coefficient value."),
    };
    let a_end = match matches.get_one::<u64>("end") {
        Some(&e) => e,
        None => panic!("No initial coefficient value."),
    };
    let table = matches.get_flag("table");
    let cycle = matches.get_flag("cycle");
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
            let mut cycle_counts = HashMap::new();
            let mut cycle_mins = Vec::new();
            let mut cycles = HashMap::new();
            (1..=n).step_by(2).for_each(|x| {
                extended_collatz(x, a, p, &mut cycle_counts, &mut cycle_mins, &mut cycles);
            });
            if table && cycles.len() > 1 {
                write_table(&cycle_mins, &n, &a);
            }
            if cycle {
                write_cycle(&cycles, &cycle_counts, &a);
            }
        });
    print_hms(&start)
}
