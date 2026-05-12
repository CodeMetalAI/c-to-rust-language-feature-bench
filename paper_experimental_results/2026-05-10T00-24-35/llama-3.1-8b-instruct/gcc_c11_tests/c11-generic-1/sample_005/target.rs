#![allow(unused_imports)]

use std::mem;

fn check<T: Default + Copy + Eq>(value: T) {
    if value != T::default() {
        panic!("exit");
    }
}

fn main() {
    let mut n: u32 = 0;

    check(_generic(n += 1, 0, n));
    check(n);

    check(_generic(n, n += 1, 0));
    check(n);

    let const_n = 0;
    check(_generic(const_n, 0, n += 1));
    check(n);
    check(_generic(mem::transmute(const_n), 0, n += 1));
    check(n);

    let mut a: [i32; 1] = [0];
    let mut ca: [i32; 1] = [0];
    check(_generic(&a, 0, n += 1));
    check(n);
    check(_generic(&ca, n += 1, 0));
    check(n);

    let f = || {};
    check(_generic(f, 0, n += 1));
    check(n);

    check(_generic(&abort, 0, n += 1));
    check(n);

    let s: i16 = 0;
    check(_generic(s, 0, n += 1));
    check(n);

    std::process::exit(0);
}

fn _generic<T, DefaultT, CopyT, EqT>(value: CopyT, default_val: DefaultT, value_val: EqT) -> EqT
where
    DefaultT: Default,
    CopyT: Copy + Eq,
    EqT: Eq,
{
    value
}