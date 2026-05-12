use std::process;

#[derive(Default)]
struct Outer {
    base: i32,
    a: i32,
    b: i32,
    deep: i32,
    c: i32,
    d: i32,
    x: i32,
    y: i32,
    tail_x: i32,
    tail_y: i32,
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let mut o = Outer::default();
    o.base = 10;
    o.a = 1;
    o.b = 2;
    o.deep = 99;
    o.c = 3;
    o.d = 4;
    o.tail_x = 7;
    o.tail_y = 8;

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
    let mut o = Outer::default();
    o.base = 0;

    o.deep = 0x11223344;
    o.x = o.deep;

    if o.x != 0x11223344 {
        return 20;
    }

    o.x = 5;
    o.y = 6;
    if o.x != 5 {
        return 21;
    }
    if o.y != 6 {
        return 22;
    }

    o.tail_p = 40;
    o.tail_q = 41;
    if o.tail_p != 40 {
        return 23;
    }
    if o.tail_q != 41 {
        return 24;
    }

    o.tail_y = -9;
    if o.tail_y != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    0
}

fn main() {
    let mut r = check_designated_init();
    if r != 0 {
        process::exit(r);
    }

    r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        process::exit(r);
    }

    r = check_addressability();
    if r != 0 {
        process::exit(r);
    }

    process::exit(0);
}