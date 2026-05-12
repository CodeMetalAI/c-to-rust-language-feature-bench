struct Outer {
    base: i32,
    inner: Inner,
    tail: Tail,
}

struct Inner {
    a: i32,
    u: U,
    d: i32,
}

union U {
    u_as_int: i32,
    s: S,
    u_as_unsigned: u32,
}

struct S {
    b: i32,
    deep: Deep,
    c: i32,
}

union Deep {
    deep: i32,
    p: P,
}

struct P {
    x: i32,
    y: i32,
}

union Tail {
    tail_i: i32,
    s: T,
}

struct T {
    tail_x: i32,
    tail_y: Y,
}

union Y {
    tail_y: i32,
    p: Q,
}

struct Q {
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        inner: Inner {
            a: 1,
            u: U { u_as_int: 0, s: S { b: 2, deep: Deep { deep: 99, p: P { x: 0, y: 0 } }, c: 3 }, u_as_unsigned: 0 },
            d: 4,
        },
        tail: Tail { tail_i: 0, s: T { tail_x: 7, tail_y: Y { tail_y: 8, p: Q { tail_p: 0, tail_q: 0 } } } },
    };

    if o.base!= 10 {
        return 1;
    }
    if o.inner.a!= 1 {
        return 2;
    }
    if o.inner.u.s.b!= 2 {
        return 3;
    }
    if o.inner.u.s.deep.deep!= 99 {
        return 4;
    }
    if o.inner.u.s.c!= 3 {
        return 5;
    }
    if o.inner.d!= 4 {
        return 6;
    }
    if o.tail.s.tail_x!= 7 {
        return 7;
    }
    if o.tail.s.tail_y.tail_y!= 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        inner: Inner {
            a: 0,
            u: U { u_as_int: 0, s: S { b: 0, deep: Deep { deep: 0, p: P { x: 0, y: 0 } }, c: 0 }, u_as_unsigned: 0 },
            d: 0,
        },
        tail: Tail { tail_i: 0, s: T { tail_x: 0, tail_y: Y { tail_y: 0, p: Q { tail_p: 0, tail_q: 0 } } } },
    };

    o.inner.u.s.deep.deep = 0x11223344;

    if o.inner.u.s.deep.p.x!= 0x11223344 {
        return 20;
    }

    o.inner.u.s.deep.p.x = 5;
    o.inner.u.s.deep.p.y = 6;
    if o.inner.u.s.deep.p.x!= 5 {
        return 21;
    }
    if o.inner.u.s.deep.p.y!= 6 {
        return 22;
    }

    o.tail.s.tail_y.p.tail_p = 40;
    o.tail.s.tail_y.p.tail_q = 41;
    if o.tail.s.tail_y.p.tail_p!= 40 {
        return 23;
    }
    if o.tail.s.tail_y.p.tail_q!= 41 {
        return 24;
    }

    o.tail.s.tail_y.tail_y = -9;
    if o.tail.s.tail_y.tail_y!= -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer {
        base: 0,
        inner: Inner {
            a: 0,
            u: U { u_as_int: 0, s: S { b: 0, deep: Deep { deep: 0, p: P { x: 0, y: 0 } }, c: 0 }, u_as_unsigned: 0 },
            d: 0,
        },
        tail: Tail { tail_i: 0, s: T { tail_x: 0, tail_y: Y { tail_y: 0, p: Q { tail_p: 0, tail_q: 0 } } } },
    };

    let pa = &o.inner.a as *const _ as usize;
    let pb = &o.inner.u.s.b as *const _ as usize;
    let pdeep = &o.inner.u.s.deep.deep as *const _ as usize;
    let ptx = &o.tail.s.tail_x as *const _ as usize;
    let pty = &o.tail.s.tail_y.tail_y as *const _ as usize;

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

    std::process::exit(0);
}