use std::collections::{HashMap, VecDeque};


fn collatz_step(n: &mut u64, a: u64, p: u64) {
    *n *= a;
    *n += p - *n & (p - 1);
    while *n & 1 == 0 {
        *n >>= 1;
    }
}

fn collatz_cycle_min(mut n: u64, a: u64, p: u64) -> (u64, VecDeque<u64>) {
    let m = n.clone();
    let mut cycle = VecDeque::new();
    while m != n || cycle.is_empty() {
        cycle.push_back(n);
        collatz_step(&mut n, a, p);
    }
    let &cycle_min = cycle.iter().min().unwrap();
    let mut front = *cycle.front().unwrap();
    while front != cycle_min {
        let _ = cycle.pop_front();
        cycle.push_back(front);
        front = *cycle.front().unwrap();
    }
    (cycle_min, cycle)
}

pub(crate) fn extended_collatz(
    n: u64,
    a: u64,
    p: u64,
    cycle_mins: &mut HashMap<u64, u64>,
    cycle_map: &mut HashMap<u64, u64>,
    cycles: &mut HashMap<u64, VecDeque<u64>>,
) -> () {
    let (mut slow, mut fast) = (n.clone(), n.clone());
    collatz_step(&mut slow, a, p);
    collatz_step(&mut fast, a, p);
    collatz_step(&mut fast, a, p);
    while slow != fast {
        collatz_step(&mut slow, a, p);
        collatz_step(&mut fast, a, p);
        collatz_step(&mut fast, a, p);
    }
    if cycle_map.contains_key(&n) {
        let cycle_min = *cycle_map.get(&n).unwrap();
        match cycle_mins.get(&cycle_min) {
            Some(v) => cycle_mins.insert(cycle_min, v + 1),
            None => cycle_mins.insert(cycle_min, 1),
        };
    } else {
        let (cycle_min, cycle) = collatz_cycle_min(slow, a, p);
        cycle_map.insert(n, cycle_min);
        cycles.insert(cycle_min, cycle.clone());
        match cycle_mins.get(&cycle_min) {
            Some(v) => cycle_mins.insert(cycle_min, v + 1),
            None => cycle_mins.insert(cycle_min, 1),
        };
    }
}
