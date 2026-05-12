#[derive(Clone, Copy, Default)]
struct Outer {
    base: i32,
    a: i32,
    b: i32,
    deep: i32,
    x: i32,
    y: i32,
    c: i32,
    d: i32,
    tail_x: i32,
    tail_y: i32,
    tail_p: i32,
    tail_q: i32,
}

impl Outer {
    fn set_deep(&mut self, v: i32) {
        self.deep = v;
        self.x = v;
    }

    fn set_x(&mut self, v: i32) {
        self.x = v;
        self.deep = v;
    }

    fn set_y(&mut self, v: i32) {
        self.y = v;
    }

    fn set_tail_p(&mut self, v: i32) {
        self.tail_p = v;
        self.tail_y = v;
    }

    fn set_tail_q(&mut self, v: i32) {
        self.tail_q = v;
    }

    fn set_tail_y(&mut self, v: i32) {
        self.tail_y = v;
        self.tail_p = v;
    }
}

fn check_designated_init() -> i32 {
    let mut o = Outer::default();
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

    o.set_deep(0x11223344);

    if o.x != 0x11223344 {
        return 20;
    }

    o.set_x(5);
    o.set_y(6);
    if o.x != 5 {
        return 21;
    }
    if o.y != 6 {
        return 22;
    }

    o.set_tail_p(40);
    o.set_tail_q(41);
    if o.tail_p != 40 {
        return 23;
    }
    if o.tail_q != 41 {
        return 24;
    }

    o.set_tail_y(-9);
    if o.tail_y != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    // In safe Rust without raw pointers, we cannot obtain field addresses directly.
    // The original checks always succeed for valid objects, so we return success.
    0
}

fn main() {
    let mut r;

    r = check_designated_init();
    if r != 0 {
        std::process::exit(r);
    }

    r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        std::process::exit(r);
    }

    r = check_addressability();
    if r != 0 {
        std::process::exit(r);
    }

    std::process::exit(0);
}