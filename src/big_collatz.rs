use crate::collatz::Unsigned;
use crate::collatz::Unsigned::{BigInteger, U128, U64};
use rug::{Assign, Integer};
use std::collections::HashMap;

fn big_collatz_step(n: &mut Integer, a: u32, p: u32) {
    *n *= a;
    *n += p - (n.mod_u(p));
    while n.mod_u(2) == 0 {
        *n >>= 1;
    }
}

fn big_collatz_cycle(n: Integer, a: u32, p: u32, cycle: &mut Vec<Unsigned>) {
    let mut m = n.clone();
    while m != n || cycle.is_empty() {
        cycle.push(
            m.to_u64().map(|x| U64(x)).unwrap_or(
                m.to_u128()
                    .map(|x| U128(x))
                    .unwrap_or(BigInteger(m.clone())),
            ),
        );
        big_collatz_step(&mut m, a, p);
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

pub fn big_collatz(
    n: u64,
    a: u32,
    p: u32,
    cycle_mins: &mut Vec<Unsigned>,
    cycles: &mut HashMap<Unsigned, Vec<Unsigned>>,
) {
    let (mut slow, mut fast) = (Integer::new(), Integer::new());
    slow.assign(n);
    fast.assign(n);
    loop {
        big_collatz_step(&mut slow, a, p);
        big_collatz_step(&mut fast, a, p);
        big_collatz_step(&mut fast, a, p);
        if slow == fast {
            break;
        }
    }
    let cycle_min = if slow < n {
        cycle_mins[slow.to_usize().unwrap() / 2usize].clone()
    } else if fast < n {
        cycle_mins[fast.to_usize().unwrap() / 2usize].clone()
    } else {
        let mut cycle = Vec::new();
        big_collatz_cycle(slow, a, p, &mut cycle);
        let cm = cycle[0].clone();
        if !cycles.contains_key(&cm) {
            cycles.insert(cm.clone(), cycle);
        }
        cm
    };
    cycle_mins.push(cycle_min)
}
