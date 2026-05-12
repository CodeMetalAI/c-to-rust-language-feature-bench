#[repr(C)]
struct Outer {
    base: i32,
    a: NestedA,
    tail: Tail,
}

#[repr(C)]
struct NestedA {
    a: i32,
    b: NestedB,
    d: i32,
}

#[repr(C)]
struct NestedB {
    b: i32,
    deep: Deep,
    c: i32,
}

#[repr(C)]
struct Deep {
    x: i32,
    y: i32,
}

#[repr(C)]
struct Tail {
    tail_x: i32,
    tail_y: TailY,
}

#[repr(C)]
struct TailY {
    tail_y: i32,
    tail_p: i32,
    tail_q: i32,
}


fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        a: NestedA {
            a: 1,
            b: NestedB {
                b: 2,
                deep: Deep { x: 99, y: 0 },
                c: 3,
            },
            d: 4,
        },
        tail: Tail {
            tail_x: 7,
            tail_y: TailY { tail_y: 8, tail_p: 0, tail_q: 0 },
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.a.a != 1 {
        return 2;
    }
    if o.a.b.b != 2 {
        return 3;
    }
    if o.a.b.deep.x != 99 {
        return 4;
    }
    if o.a.b.c != 3 {
        return 5;
    }
    if o.a.d != 4 {
        return 6;
    }
    if o.tail.tail_x != 7 {
        return 7;
    }
    if o.tail.tail_y.tail_y != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        a: NestedA {
            a: 0,
            b: NestedB {
                b: 0,
                deep: Deep { x: 0, y: 0 },
                c: 0,
            },
            d: 0,
        },
        tail: Tail {
            tail_x: 0,
            tail_y: TailY { tail_y: 0, tail_p: 0, tail_q: 0 },
        },
    };

    o.a.b.deep.x = 0x11223344;

    if o.a.b.deep.x != 0x11223344 {
        return 20;
    }

    o.a.b.deep.x = 5;
    o.a.b.deep.y = 6;

    if o.a.b.deep.x != 5 {
        return 21;
    }
    if o.a.b.deep.y != 6 {
        return 22;
    }

    o.tail.tail_y.tail_p = 40;
    o.tail.tail_y.tail_q = 41;

    if o.tail.tail_y.tail_p != 40 {
        return 23;
    }
    if o.tail.tail_y.tail_q != 41 {
        return 24;
    }

    o.tail.tail_y.tail_y = -9;

    if o.tail.tail_y.tail_y != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer {
        base: 0,
        a: NestedA {
            a: 0,
            b: NestedB {
                b: 0,
                deep: Deep { x: 0, y: 0 },
                c: 0,
            },
            d: 0,
        },
        tail: Tail {
            tail_x: 0,
            tail_y: TailY { tail_y: 0, tail_p: 0, tail_q: 0 },
        },
    };

    let pa = &o.a as *const NestedA as uptr;
    let pb = &o.a.b as *const NestedB as uptr;
    let pdeep = &o.a.b.deep as *const Deep as uptr;
    let ptx = &o.tail.tail_x as *const i32 as uptr;
    let pty = &o.tail.tail_y as *const TailY as uptr;

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

type uptr = u64;

fn main() {
    let mut r = check_designated_init();
    if r != 0 {
        std::process::exit(r as i32);
    }

    r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        std::process::exit(r as i32);
    }

    r = check_addressability();
    if r != 0 {
        std::process::exit(r as i32);
    }

    std::process::exit(0);
}