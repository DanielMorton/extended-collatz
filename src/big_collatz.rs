use std::collections::VecDeque;
use rug::{Assign, Integer};

fn big_collatz_step(n: &mut Integer, a: u32, p: u32) {
    *n *= a;
    *n += p - (n.mod_u(p));
    while n.mod_u(2) == 0 {
        *n >>= 1;
    }
}

fn big_collatz_cycle(n: Integer, a: u32, p: u32) -> u64 {
    let mut m = n.clone();
    let mut cycle = VecDeque::new();
    while m != n || cycle.is_empty() {
        cycle.push_back(m.clone());
        big_collatz_step(&mut m, a, p);
    }
    cycle.iter().min().unwrap().to_u64().unwrap()

}

pub fn big_collatz(n: u64, a: u32, p: u32) -> u64 {
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
    big_collatz_cycle(slow, a, p)
}
