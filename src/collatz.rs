use std::collections::{HashMap, VecDeque};

fn collatz_step64(n: &mut u64, a: u64, p: u64) {
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

fn collatz_step128(n: &mut u128, a: u128, p: u128) {
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

fn collatz_cycle(n: &u64, a: u64, p: u64, cycle: &mut VecDeque<u64>) -> () {
    let mut m = *n;
    while &m != n || cycle.is_empty() {
        cycle.push_back(m);
        collatz_step64(&mut m, a, p);
        if m == 0 {
            cycle.clear();
            return;
        }
    }
    let &cycle_min = cycle.iter().min().unwrap();
    let mut front = *cycle.front().unwrap();
    while front != cycle_min {
        let _ = cycle.pop_front();
        cycle.push_back(front);
        front = *cycle.front().unwrap();
    }
}

fn collatz_cycle128(n: &u128, a: u128, p: u128, cycle: &mut VecDeque<u128>) -> () {
    let mut m = *n;
    while &m != n || cycle.is_empty() {
        cycle.push_back(m);
        collatz_step128(&mut m, a, p);
        if m == 0 {
            cycle.clear();
            return;
        }
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
) -> bool {
    let (mut slow, mut fast) = (n, n);
    loop {
        collatz_step64(&mut slow, a, p);
        collatz_step64(&mut fast, a, p);
        collatz_step64(&mut fast, a, p);
        if slow == 0 || fast == 0 {
            cycle_mins.push(0);
            return false;
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
        let mut cycle = VecDeque::new();
        collatz_cycle(&slow, a, p, &mut cycle);
        let &cm = cycle.front().unwrap();
        if !cycles.contains_key(&cm) {
            cycles.insert(cm, cycle);
        }
        cm
    };
    cycle_mins.push(cycle_min);
    if cycle_min == 0 {
        return false;
    }
    match cycle_counts.get(&cycle_min) {
        Some(v) => cycle_counts.insert(cycle_min, v + 1),
        None => cycle_counts.insert(cycle_min, 1),
    };
    true
}

pub fn extended_collatz128(
    n: u128,
    a: u128,
    p: u128,
    cycle_counts: &mut HashMap<u64, u64>,
    cycle_mins: &mut Vec<u64>,
    cycles: &mut HashMap<u64, VecDeque<u128>>,
) -> bool {
    let (mut slow, mut fast) = (n, n);
    loop {
        collatz_step128(&mut slow, a, p);
        collatz_step128(&mut fast, a, p);
        collatz_step128(&mut fast, a, p);
        if slow == 0 || fast == 0 {
            return false;
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
        let mut cycle = VecDeque::new();
        collatz_cycle128(&slow, a, p, &mut cycle);
        let cm = *cycle.front().unwrap() as u64;
        if !cycles.contains_key(&cm) {
            cycles.insert(cm, cycle);
        }
        cm
    };
    cycle_mins[(n / 2) as usize] = cycle_min;
    if cycle_min == 0 {
        return false;
    }
    match cycle_counts.get(&cycle_min) {
        Some(v) => cycle_counts.insert(cycle_min, v + 1),
        None => cycle_counts.insert(cycle_min, 1),
    };
    true
}
