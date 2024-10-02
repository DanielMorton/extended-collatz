use anyhow::Result;
use log::info;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;
use std::time::Instant;

mod collatz;
mod parse;
mod write;

use crate::collatz::extended_collatz;
use crate::parse::Args;
use crate::write::{write_cycle, write_table};

fn print_elapsed_time(start: &Instant) {
    let millis = start.elapsed().as_millis();
    let seconds = millis / 1000;
    let (hour, minute, second) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    info!(
        "Elapsed time: {:02}:{:02}:{:02}.{:03}",
        hour,
        minute,
        second,
        millis % 1000
    );
}

fn process_collatz(
    a: u64,
    n: u64,
    should_write_table: bool,
    should_write_cycle: bool,
) -> Result<()> {
    let p = (a - 1).next_power_of_two();
    let mut cycle_mins = Vec::new();
    let mut cycles = HashMap::new();

    (1..=n).step_by(2).for_each(|x| {
        extended_collatz(x, a, p, &mut cycle_mins, &mut cycles);
    });

    if should_write_table && cycles.len() > 1 {
        write_table(&cycle_mins, n, a)?;
    }

    if should_write_cycle {
        let cycle_counts = cycle_mins.iter().fold(HashMap::new(), |mut acc, cm| {
            *acc.entry(cm).or_insert(0) += 1;
            acc
        });
        write_cycle(&cycles, &cycle_counts, a)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse()?;
    let start = Instant::now();

    (args.a_start..=args.a_end)
        .into_par_iter()
        .filter(|&a| a % 2 == 1)
        .try_for_each(|a| process_collatz(a, args.n, args.write_table, args.write_cycle))?;

    print_elapsed_time(&start);
    Ok(())
}
