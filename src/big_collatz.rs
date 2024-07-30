use rug::{Assign, Integer};

fn big_collatz_step(n: &mut Integer, a: u32, p: u32) {
    *n *= a;
    *n += p - (n.mod_u(p));
    while n.mod_u(2) == 0 {
        *n >>= 1;
    }
}

pub fn big_collatz(n: u64, a: u32, p: u32) -> u64 {
    let (mut slow, mut fast) = (Integer::new(), Integer::new());
    slow.assign(n);
    fast.assign(n);
    loop {
        big_collatz_step(&mut slow, a, p);
        big_collatz_step(&mut fast, a, p);
        big_collatz_step(&mut fast, a , p);
        if slow == fast {
            break;
        }
    }
    slow.to_u64().unwrap()
}