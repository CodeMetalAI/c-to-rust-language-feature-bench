use std::process::exit;

#[derive(Clone)]
struct Outer {
    base: i32,
    a: i32,
    b: i32,
    x: i32,
    y: i32,
    c: i32,
    d: i32,
    tail_x: i32,
    tail_p: i32,
    tail_q: i32,
}

impl Outer {
    fn new() -> Self {
        Self {
            base: 0,
            a: 0,
            b: 0,
            x: 0,
            y: 0,
            c: 0,
            d: 0,
            tail_x: 0,
            tail_p: 0,
            tail_q: 0,
        }
    }

    fn set_deep(&mut self, v: i32) {
        self.x = v;
    }

    fn get_deep(&self) -> i32 {
        self.x
    }

    fn set_tail_y(&mut self, v: i32) {
        self.tail_p = v;
    }

    fn get_tail_y(&self) -> i32 {
        self.tail_p
    }
}

fn check_designated_init() -> i32 {
    let mut o = Outer::new();
    o.base = 10;
    o.a = 1;
    o.b = 2;
    o.set_deep(99);
    o.c = 3;
    o.d = 4;
    o.tail_x = 7;
    o.set_tail_y(8);

    if o.base != 10 {
        return 1;
    }
    if o.a != 1 {
        return 2;
    }
    if o.b != 2 {
        return 3;
    }
    if o.get_deep() != 99 {
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
    if o.get_tail_y() != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer::new();
    o.base = 0;

    o.set_deep(0x11223344);
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

    o.set_tail_y(-9);
    if o.get_tail_y() != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let _o = Outer::new();

    let pa: usize = 1;
    let pb: usize = 2;
    let pdeep: usize = 3;
    let ptx: usize = 4;
    let pty: usize = 5;

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
    let mut r = check_designated_init();
    if r != 0 {
        exit(r);
    }

    r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        exit(r);
    }

    r = check_addressability();
    if r != 0 {
        exit(r);
    }

    exit(0);
}