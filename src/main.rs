#![allow(warnings)]
use crate::big_collatz::big_collatz;
use crate::collatz::{extended_collatz, extended_collatz128};
use crate::write::{write_cycle, write_table};
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;
use std::time::Instant;

mod big_collatz;
mod collatz;
mod parse;
mod write;

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
        None => panic!("No final coefficient value."),
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
            let mut cycles128 = HashMap::new();
            let mut list128 = Vec::new();
            let mut list_big = Vec::new();
            (1..=n).step_by(2).for_each(|x| {
                let finished =
                    extended_collatz(x, a, p, &mut cycle_counts, &mut cycle_mins, &mut cycles);
                if !finished {
                    list128.push(x as u128);
                }
            });
            list128.iter().for_each(|&x| {
                let finished = extended_collatz128(
                    x,
                    a as u128,
                    p as u128,
                    &mut cycle_counts,
                    &mut cycle_mins,
                    &mut cycles128,
                );
                if !finished {
                    list_big.push(x as u64);
                }
            });
            list_big.iter().for_each(|&x| {
                let cycle_min = big_collatz(x, a as u32, p as u32);
                cycle_mins[(x / 2) as usize] = cycle_min;
                match cycle_counts.get(&cycle_min) {
                    Some(v) => cycle_counts.insert(cycle_min, v + 1),
                    None => cycle_counts.insert(cycle_min, 1),
                };
            });
            if table && cycles.len() > 1 {
                write_table(&cycle_mins, &n, &a);
            }
            if cycle {
                write_cycle(&cycles, &cycles128, &cycle_counts, &a);
            }
        });
    print_hms(&start)
}
