#![allow(unused_parens)]

use std::mem;

type Uptr = usize;

struct Outer {
    base: i32,
    a: i32,
    b: i32,
    deep: i32,
    c: i32,
    d: i32,
    tail_x: i32,
    tail_y: i32,
}

impl Outer {
    fn new() -> Self {
        Outer {
            base: 0,
            a: 0,
            b: 0,
            deep: 0,
            c: 0,
            d: 0,
            tail_x: 0,
            tail_y: 0,
        }
    }
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        a: 1,
        b: 2,
        deep: 99,
        c: 3,
        d: 4,
        tail_x: 7,
        tail_y: 8,
    };

    if o.base != 10 {
        return 1;
    }
    if o.a != 1 {
        return 2;
    }
    if o.b != 2 {
        return 3;
    }
    if o.deep != 99 {
        return 4;
    }
    if o.c != 3 {
        return 5;
    }
    if o.d != 4 {
        return 6;
    }
    if o.tail_x != 7 {
        return 7;
    }
    if o.tail_y != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer::new();
    o.base = 0;

    o.deep = 0x11223344;

    if o.deep != 0x11223344 {
        return 20;
    }

    o.deep = 5;
    o.deep = 6;
    if o.deep != 6 {
        return 21;
    }

    o.tail_x = 40;
    o.tail_y = 41;
    if o.tail_x != 40 {
        return 23;
    }
    if o.tail_y != 41 {
        return 24;
    }

    o.tail_y = -9;
    if o.tail_y != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer::new();

    let pa = &o.a as *const i32 as Uptr;
    let pb = &o.b as *const i32 as Uptr;
    let pdeep = &o.deep as *const i32 as Uptr;
    let ptx = &o.tail_x as *const i32 as Uptr;
    let pty = &o.tail_y as *const i32 as Uptr;

    if pa == 0 || pb == 0 || pdeep == 0 || ptx == 0 || pty == 0 {
        return 30;
    }
    if pa == pb {
        return 31;
    }
    if ptx == pty {
        return 32;
    }

    0
}

fn main() {
    let r = check_designated_init();
    if r != 0 {
        std::process::exit(r);
    }

    let r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        std::process::exit(r);
    }

    let r = check_addressability();
    if r != 0 {
        std::process::exit(r);
    }

    std::process::exit(0);
}