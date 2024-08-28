use std::collections::HashMap;

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

fn collatz_cycle(n: &u64, a: u64, p: u64, cycle: &mut Vec<u64>) -> () {
    let mut m = *n;
    while &m != n || cycle.is_empty() {
        cycle.push(m);
        collatz_step64(&mut m, a, p);
        if m == 0 {
            cycle.clear();
            return;
        }
    }
    let min_id = cycle
        .iter()
        .enumerate()
        .min_by_key(|(_, &v)| v)
        .map(|(i, _)| i)
        .unwrap();
    let mut cycle_back = cycle[..min_id].to_vec();
    *cycle = cycle[min_id..].to_owned();
    cycle.append(&mut cycle_back);
    /*let &cycle_min = cycle.iter().min().unwrap();
    let mut front = *cycle.front().unwrap();
    while front != cycle_min {
        let _ = cycle.pop_front();
        cycle.push_back(front);
        front = *cycle.front().unwrap();
    }*/
}

fn collatz_cycle128(n: &u128, a: u128, p: u128, cycle: &mut Vec<u128>) -> () {
    let mut m = *n;
    while &m != n || cycle.is_empty() {
        cycle.push(m);
        collatz_step128(&mut m, a, p);
        if m == 0 {
            cycle.clear();
            return;
        }
    }
    let min_id = cycle
        .iter()
        .enumerate()
        .min_by_key(|(_, &v)| v)
        .map(|(i, _)| i)
        .unwrap();
    let mut cycle_back = cycle[..min_id].to_vec();
    *cycle = cycle[min_id..].to_owned();
    cycle.append(&mut cycle_back);
    /*let &cycle_min = cycle.iter().min().unwrap();
    let mut front = *cycle.front().unwrap();
    while front != cycle_min {
        let _ = cycle.pop_front();
        cycle.push_back(front);
        front = *cycle.front().unwrap();
    }*/
}

pub fn extended_collatz(
    n: u64,
    a: u64,
    p: u64,
    cycle_mins: &mut Vec<u64>,
    cycles: &mut HashMap<u64, Vec<u64>>,
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
        let mut cycle = Vec::new();
        collatz_cycle(&slow, a, p, &mut cycle);
        let cm = cycle[0];
        if !cycles.contains_key(&cm) {
            cycles.insert(cm, cycle);
        }
        cm
    };
    cycle_mins.push(cycle_min);
    cycle_min > 0
}

pub fn extended_collatz128(
    n: u128,
    a: u128,
    p: u128,
    cycle_mins: &mut Vec<u64>,
    cycles: &mut HashMap<u64, Vec<u128>>,
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
        let mut cycle = Vec::new();
        collatz_cycle128(&slow, a, p, &mut cycle);
        let cm = cycle[0] as u64;
        if !cycles.contains_key(&cm) {
            cycles.insert(cm, cycle);
        }
        cm
    };
    cycle_mins[(n / 2) as usize] = cycle_min;
    cycle_min > 0
}
