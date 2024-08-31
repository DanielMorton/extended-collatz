use crate::collatz::Unsigned::{BigInteger, U128, U64};
use rug::{Assign, Integer};
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum Unsigned {
    U64(u64),
    U128(u128),
    BigInteger(Integer),
}

fn is_even(n: &Unsigned) -> bool {
    match n {
        U64(u) => u & 1 == 0,
        U128(u) => u & 1 == 0,
        BigInteger(u) => u.mod_u(2) == 0,
    }
}

fn collatz_step(n: &mut Unsigned, a: u64, p: u64) {
    *n = match n {
        U64(u) => match u.checked_mul(a) {
            Some(m) => U64(m),
            None => U128((*u as u128) * (a as u128)),
        },
        U128(u) => match u.checked_mul(a as u128) {
            Some(m) => U128(m),
            None => {
                let mut i = Integer::new();
                i.assign(*u);
                BigInteger(i * a)
            }
        },
        BigInteger(u) => BigInteger(u.clone() * a),
    };
    *n = match n {
        U64(u) => match u.checked_add(p - *u & (p - 1)) {
            Some(m) => U64(m),
            None => U128((*u as u128) + (p - *u & (p - 1)) as u128),
        },
        U128(u) => {
            let p128 = p as u128;
            match u.checked_add((p128 - *u & (p128 - 1))) {
                Some(m) => U128(m),
                None => {
                    let mut i = Integer::new();
                    i.assign(*u);
                    BigInteger(i + (p128 - (*u & (p128 - 1))) as u64)
                }
            }
        }
        BigInteger(u) => BigInteger(u.clone() + (p as u32) - u.mod_u(p as u32)),
    };
    while is_even(n) {
        *n = match n {
            U64(u) => U64(*u >> 1),
            U128(u) => {
                let v = *u >> 1;
                u64::try_from(v).map(|x| U64(x)).unwrap_or(U128(v))
            }
            BigInteger(u) => {
                let v: Integer = u.clone() >> 1;
                v.to_u128().map(|x| U128(x)).unwrap_or(BigInteger(v))
            }
        }
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

fn collatz_cycle(n: &Unsigned, a: u64, p: u64, cycle: &mut Vec<Unsigned>) -> () {
    let mut m = n.clone();
    while &m != n || cycle.is_empty() {
        cycle.push(m.clone());
        collatz_step(&mut m, a, p);
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
) {
    let (mut slow, mut fast, un) = (U64(n), U64(n), U64(n));
    loop {
        collatz_step(&mut slow, a, p);
        collatz_step(&mut fast, a, p);
        collatz_step(&mut fast, a, p);
        if slow == fast || slow < un || fast < un {
            break;
        }
    }
    let cycle_min = if slow < un {
        if let U64(m) = slow {
            cycle_mins[(m / 2) as usize].clone()
        } else {
            panic!()
        }
    } else if fast < un {
        if let U64(m) = fast {
            cycle_mins[(m / 2) as usize].clone()
        } else {
            panic!()
        }
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
}
