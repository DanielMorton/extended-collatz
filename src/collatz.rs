use std::collections::{HashMap, VecDeque};

fn collatz_step(n: &mut u64, a: u64, p: u64) {
    *n = n.checked_mul(a).unwrap_or(0);
    if *n == 0 {
        return;
    }
    *n = n.checked_add(p - *n & (p - 1)).unwrap_or(0);
    if *n == 0 {
        return;
    }
    while *n & 1 == 0 {
        *n >>= 1;
    }
}

fn collatz_cycle_min(n: &u64, a: u64, p: u64, cycle: &mut VecDeque<u64>) -> () {
    let mut m = *n;
    while &m != n || cycle.is_empty() {
        cycle.push_back(m);
        collatz_step(&mut m, a, p);
    }
    let &cycle_min = cycle.iter().min().unwrap();
    let mut front = *cycle.front().unwrap();
    while front != cycle_min {
        let _ = cycle.pop_front();
        cycle.push_back(front);
        front = *cycle.front().unwrap();
    }
}

pub fn extended_collatz(
    n: u64,
    a: u64,
    p: u64,
    cycle_counts: &mut HashMap<u64, u64>,
    cycle_mins: &mut Vec<u64>,
    cycles: &mut HashMap<u64, VecDeque<u64>>,
) -> () {
    let (mut slow, mut fast) = (n, n);
    loop {
        collatz_step(&mut slow, a, p);
        collatz_step(&mut fast, a, p);
        collatz_step(&mut fast, a, p);
        if slow == 0 || fast == 0 {
            cycle_mins.push(0);
            return;
        }
        if slow == fast || slow < n || fast < n {
            break;
        }
    }
    let cycle_min = if slow < n {
        cycle_mins[(slow / 2) as usize]
    } else if fast < n {
        cycle_mins[(fast / 2) as usize]
    } else {
        let mut cycle =VecDeque::new();
        collatz_cycle_min(&slow, a, p, &mut cycle);
        let &cm = cycle.front().unwrap();
        if !cycles.contains_key(&cm) {
            cycles.insert(cm, cycle.clone());
        }
        cm
    };
    cycle_mins.push(cycle_min);
    match cycle_counts.get(&cycle_min) {
        Some(v) => cycle_counts.insert(cycle_min, v + 1),
        None => cycle_counts.insert(cycle_min, 1),
    };
}
