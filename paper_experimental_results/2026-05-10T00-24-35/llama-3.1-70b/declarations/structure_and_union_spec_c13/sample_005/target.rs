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

    if o.base!= 10 {
        return 1;
    }
    if o.a!= 1 {
        return 2;
    }
    if o.b!= 2 {
        return 3;
    }
    if o.deep!= 99 {
        return 4;
    }
    if o.c!= 3 {
        return 5;
    }
    if o.d!= 4 {
        return 6;
    }
    if o.tail_x!= 7 {
        return 7;
    }
    if o.tail_y!= 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer::default();
    o.deep = 0x11223344;

    if o.deep!= 0x11223344 {
        return 20;
    }

    o.x = 5;
    o.y = 6;
    if o.x!= 5 {
        return 21;
    }
    if o.y!= 6 {
        return 22;
    }

    o.tail_p = 40;
    o.tail_q = 41;
    if o.tail_p!= 40 {
        return 23;
    }
    if o.tail_q!= 41 {
        return 24;
    }

    o.tail_y = -9;
    if o.tail_y!= -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer::default();

    let pa = &o.a as *const _ as usize;
    let pb = &o.b as *const _ as usize;
    let pdeep = &o.deep as *const _ as usize;
    let ptx = &o.tail_x as *const _ as usize;
    let pty = &o.tail_y as *const _ as usize;

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
    if r!= 0 {
        std::process::exit(r);
    }

    let r = check_union_aliasing_via_flattened_names();
    if r!= 0 {
        std::process::exit(r);
    }

    let r = check_addressability();
    if r!= 0 {
        std::process::exit(r);
    }
}