use std::process::exit;

type Uptr = usize;

#[derive(Clone, Copy)]
struct Outer {
    base: i32,
    a: i32,
    b: i32,
    deep_x: i32,
    y: i32,
    c: i32,
    d: i32,
    tail_x: i32,
    tail_y_p: i32,
    tail_q: i32,
}

impl Outer {
    fn new() -> Self {
        Self {
            base: 0,
            a: 0,
            b: 0,
            deep_x: 0,
            y: 0,
            c: 0,
            d: 0,
            tail_x: 0,
            tail_y_p: 0,
            tail_q: 0,
        }
    }

    fn set_deep(&mut self, v: i32) {
        self.deep_x = v;
    }

    fn get_deep(&self) -> i32 {
        self.deep_x
    }

    fn set_x(&mut self, v: i32) {
        self.deep_x = v;
    }

    fn get_x(&self) -> i32 {
        self.deep_x
    }

    fn set_y(&mut self, v: i32) {
        self.y = v;
    }

    fn get_y(&self) -> i32 {
        self.y
    }

    fn set_tail_p(&mut self, v: i32) {
        self.tail_y_p = v;
    }

    fn get_tail_p(&self) -> i32 {
        self.tail_y_p
    }

    fn set_tail_q(&mut self, v: i32) {
        self.tail_q = v;
    }

    fn get_tail_q(&self) -> i32 {
        self.tail_q
    }

    fn set_tail_y(&mut self, v: i32) {
        self.tail_y_p = v;
    }

    fn get_tail_y(&self) -> i32 {
        self.tail_y_p
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

    if o.get_x() != 0x11223344 {
        return 20;
    }

    o.set_x(5);
    o.set_y(6);
    if o.get_x() != 5 {
        return 21;
    }
    if o.get_y() != 6 {
        return 22;
    }

    o.set_tail_p(40);
    o.set_tail_q(41);
    if o.get_tail_p() != 40 {
        return 23;
    }
    if o.get_tail_q() != 41 {
        return 24;
    }

    o.set_tail_y(-9);
    if o.get_tail_y() != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer::new();

    let pa = (&o.a as *const i32 as Uptr);
    let pb = (&o.b as *const i32 as Uptr);
    let pdeep = (&o.deep_x as *const i32 as Uptr);
    let ptx = (&o.tail_x as *const i32 as Uptr);
    let pty = (&o.tail_y_p as *const i32 as Uptr);

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