use rug::Integer;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Unsigned {
    U64(u64),
    U128(u128),
    BigInteger(Integer),
    Zero,
}

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

fn collatz_cycle(n: &u64, a: u64, p: u64, cycle: &mut Vec<Unsigned>) -> () {
    let mut m = *n;
    while &m != n || cycle.is_empty() {
        cycle.push(Unsigned::U64(m));
        collatz_step64(&mut m, a, p);
        if m == 0 {
            cycle.clear();
            return;
        }
    }
    let min_id = cycle
        .iter()
        .enumerate()
        .min_by_key(|(_, &ref v)| v)
        .map(|(i, _)| i)
        .unwrap();
    let mut cycle_back = cycle[..min_id].to_vec();
    *cycle = cycle[min_id..].to_owned();
    cycle.append(&mut cycle_back);
}

fn collatz_cycle128(n: &u128, a: u128, p: u128, cycle: &mut Vec<Unsigned>) -> () {
    let mut m = *n;
    while &m != n || cycle.is_empty() {
        cycle.push(
            u64::try_from(m)
                .map(|x| Unsigned::U64(x))
                .unwrap_or(Unsigned::U128(m)),
        );
        collatz_step128(&mut m, a, p);
        if m == 0 {
            cycle.clear();
            return;
        }
    }
    let min_id = cycle
        .iter()
        .enumerate()
        .min_by_key(|(_, &ref v)| v)
        .map(|(i, _)| i)
        .unwrap();
    let mut cycle_back = cycle[..min_id].to_vec();
    *cycle = cycle[min_id..].to_owned();
    cycle.append(&mut cycle_back);
}

pub fn extended_collatz(
    n: u64,
    a: u64,
    p: u64,
    cycle_mins: &mut Vec<Unsigned>,
    cycles: &mut HashMap<Unsigned, Vec<Unsigned>>,
) -> bool {
    let (mut slow, mut fast) = (n, n);
    loop {
        collatz_step64(&mut slow, a, p);
        collatz_step64(&mut fast, a, p);
        collatz_step64(&mut fast, a, p);
        if slow == 0 || fast == 0 {
            cycle_mins.push(Unsigned::Zero);
            return false;
        }
        if slow == fast || slow < n || fast < n {
            break;
        }
    }
    let cycle_min = if slow < n {
        cycle_mins[(slow / 2) as usize].clone()
    } else if fast < n {
        cycle_mins[(fast / 2) as usize].clone()
    } else {
        let mut cycle = Vec::new();
        collatz_cycle(&slow, a, p, &mut cycle);
        let cm = cycle[0].clone();
        if !cycles.contains_key(&cm) {
            cycles.insert(cm.clone(), cycle);
        }
        cm
    };
    cycle_mins.push(cycle_min.clone());
    cycle_min != Unsigned::Zero
}

pub fn extended_collatz128(
    n: u128,
    a: u128,
    p: u128,
    cycle_mins: &mut Vec<Unsigned>,
    cycles: &mut HashMap<Unsigned, Vec<Unsigned>>,
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
        cycle_mins[(slow / 2) as usize].clone()
    } else if fast < n {
        cycle_mins[(fast / 2) as usize].clone()
    } else {
        let mut cycle = Vec::new();
        collatz_cycle128(&slow, a, p, &mut cycle);
        let cm = cycle[0].clone();
        if !cycles.contains_key(&cm) {
            cycles.insert(cm.clone(), cycle);
        }
        cm
    };
    cycle_mins[(n / 2) as usize] = cycle_min.clone();
    cycle_min != Unsigned::Zero
}
