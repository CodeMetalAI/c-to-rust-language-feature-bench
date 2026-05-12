struct Outer {
    base: i32,
    a: i32,
    u_as_int: i32,
    b: i32,
    deep: i32,
    x: i32,
    y: i32,
    c: i32,
    d: i32,
    tail_i: i32,
    tail_x: i32,
    tail_y: i32,
    tail_p: i32,
    tail_q: i32,
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
        x: 0,
        y: 0,
        u_as_int: 0,
        tail_i: 0,
        tail_p: 0,
        tail_q: 0,
    };

    if o.base != 10 {
        1
    } else if o.a != 1 {
        2
    } else if o.b != 2 {
        3
    } else if o.deep != 99 {
        4
    } else if o.c != 3 {
        5
    } else if o.d != 4 {
        6
    } else if o.tail_x != 7 {
        7
    } else if o.tail_y != 8 {
        8
    } else {
        0
    }
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        u_as_int: 0,
        deep: 0x11223344,
        x: 5,
        y: 6,
        tail_i: 0,
        tail_x: 0,
        tail_y: -9,
        tail_p: 40,
        tail_q: 41,
    };

    if o.x != 0x11223344 {
        20
    } else {
        o.x = 5;
        o.y = 6;
        if o.x != 5 {
            21
        } else if o.y != 6 {
            22
        } else {
            o.tail_p = 40;
            o.tail_q = 41;
            if o.tail_p != 40 {
                23
            } else if o.tail_q != 41 {
                24
            } else if o.tail_y != -9 {
                25
            } else {
                0
            }
        }
    }
}

fn check_addressability() -> i32 {
    let mut o = Outer {
        base: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        u_as_int: 0,
        deep: 0,
        x: 0,
        y: 0,
        tail_i: 0,
        tail_x: 0,
        tail_y: 0,
        tail_p: 0,
        tail_q: 0,
    };

    let pa = &o.a as *const i32 as usize;
    let pb = &o.b as *const i32 as usize;
    let pdeep = &o.deep as *const i32 as usize;
    let ptx = &o.tail_x as *const i32 as usize;
    let pty = &o.tail_y as *const i32 as usize;

    if pa == 0 || pb == 0 || pdeep == 0 || ptx == 0 || pty == 0 {
        30
    } else if pa == pb {
        31
    } else if ptx == pty {
        32
    } else {
        0
    }
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
}