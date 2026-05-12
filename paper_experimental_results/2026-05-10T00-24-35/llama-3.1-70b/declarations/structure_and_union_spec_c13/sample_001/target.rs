#[derive(Debug, PartialEq)]
struct Inner {
    b: i32,
    deep: i32,
    c: i32,
}

#[derive(Debug, PartialEq)]
union Middle {
    u_as_int: i32,
    inner: Inner,
    u_as_unsigned: u32,
}

#[derive(Debug, PartialEq)]
struct OuterInner {
    a: i32,
    middle: Middle,
    d: i32,
}

#[derive(Debug, PartialEq)]
struct TailInner {
    tail_p: i32,
    tail_q: i32,
}

#[derive(Debug, PartialEq)]
union Tail {
    tail_i: i32,
    tail_x: i32,
    tail_y: i32,
    inner: TailInner,
}

#[derive(Debug, PartialEq)]
struct Outer {
    base: i32,
    inner: OuterInner,
    tail: Tail,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        inner: OuterInner {
            a: 1,
            middle: Middle {
                u_as_int: 0,
                inner: Inner { b: 2, deep: 99, c: 3 },
                u_as_unsigned: 0,
            },
            d: 4,
        },
        tail: Tail {
            tail_i: 0,
            tail_x: 7,
            tail_y: 8,
            inner: TailInner { tail_p: 0, tail_q: 0 },
        },
    };

    if o.base!= 10 {
        return 1;
    }
    if o.inner.a!= 1 {
        return 2;
    }
    if o.inner.middle.inner.b!= 2 {
        return 3;
    }
    if o.inner.middle.inner.deep!= 99 {
        return 4;
    }
    if o.inner.middle.inner.c!= 3 {
        return 5;
    }
    if o.inner.d!= 4 {
        return 6;
    }
    if o.tail.tail_x!= 7 {
        return 7;
    }
    if o.tail.tail_y!= 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        inner: OuterInner {
            a: 0,
            middle: Middle {
                u_as_int: 0,
                inner: Inner { b: 0, deep: 0, c: 0 },
                u_as_unsigned: 0,
            },
            d: 0,
        },
        tail: Tail {
            tail_i: 0,
            tail_x: 0,
            tail_y: 0,
            inner: TailInner { tail_p: 0, tail_q: 0 },
        },
    };

    o.inner.middle.inner.deep = 0x11223344;

    if o.inner.middle.inner.deep!= 0x11223344 {
        return 20;
    }

    o.inner.middle.inner.deep = 5;
    o.inner.middle.inner.c = 6;
    if o.inner.middle.inner.deep!= 5 {
        return 21;
    }
    if o.inner.middle.inner.c!= 6 {
        return 22;
    }

    o.tail.inner.tail_p = 40;
    o.tail.inner.tail_q = 41;
    if o.tail.inner.tail_p!= 40 {
        return 23;
    }
    if o.tail.inner.tail_q!= 41 {
        return 24;
    }

    o.tail.tail_y = -9;
    if o.tail.tail_y!= -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer {
        base: 0,
        inner: OuterInner {
            a: 0,
            middle: Middle {
                u_as_int: 0,
                inner: Inner { b: 0, deep: 0, c: 0 },
                u_as_unsigned: 0,
            },
            d: 0,
        },
        tail: Tail {
            tail_i: 0,
            tail_x: 0,
            tail_y: 0,
            inner: TailInner { tail_p: 0, tail_q: 0 },
        },
    };

    let pa = &o.inner.a as *const _ as usize;
    let pb = &o.inner.middle.inner.b as *const _ as usize;
    let pdeep = &o.inner.middle.inner.deep as *const _ as usize;
    let ptx = &o.tail.tail_x as *const _ as usize;
    let pty = &o.tail.tail_y as *const _ as usize;

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