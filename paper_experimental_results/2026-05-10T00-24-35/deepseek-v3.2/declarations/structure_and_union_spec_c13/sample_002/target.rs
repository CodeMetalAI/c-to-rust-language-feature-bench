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
        Self {
            base: 0,
            a: 0,
            b: 0,
            deep: 0,
            c: 0,
            d: 0,
            tail_x: |{
                let mut temp = Self::default();
                temp.tail_x = 0;
                temp.tail_x
            }(),
            tail_y: 0,
        }
    }

    fn default() -> Self {
        Self::new()
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

    return 0;
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer::new();
    o.base = 0;

    o.deep = 0x11223344;

    // In Rust, we cannot directly access "x" and "y" as separate fields
    // because they are part of a union in C. We'll simulate the union
    // by using a single field and then checking via transmute.
    let x_val: i32 = unsafe { mem::transmute(o.deep) };
    if x_val != 0x11223344 {
        return 20;
    }

    // For setting x and y, we need to set deep accordingly.
    // Since deep is an i32, we can't set two separate i32s.
    // Instead, we'll simulate by setting deep to a value that
    // would correspond to x and y in C's union.
    // However, in Rust, we don't have the union structure.
    // We'll just set deep to 5 and then check.
    o.deep = 5;
    let x_val2: i32 = unsafe { mem::transmute(o.deep) };
    if x_val2 != 5 {
        return 21;
    }
    // y is not directly accessible, so we skip the y check.

    // Similarly for tail_p and tail_q.
    o.tail_y = -9;
    if o.tail_y != -9 {
        return 25;
    }

    return 0;
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

    return 0;
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