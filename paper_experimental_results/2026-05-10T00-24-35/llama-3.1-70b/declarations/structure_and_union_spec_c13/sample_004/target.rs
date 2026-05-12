fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        inner: Inner {
            a: 1,
            b: 2,
            deep: 99,
            c: 3,
            d: 4,
        },
        tail: Tail {
            x: 7,
            y: 8,
        },
    };

    if o.base!= 10 {
        return 1;
    }
    if o.inner.a!= 1 {
        return 2;
    }
    if o.inner.b!= 2 {
        return 3;
    }
    if o.inner.deep!= 99 {
        return 4;
    }
    if o.inner.c!= 3 {
        return 5;
    }
    if o.inner.d!= 4 {
        return 6;
    }
    if o.tail.x!= 7 {
        return 7;
    }
    if o.tail.y!= 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        inner: Inner {
            a: 0,
            b: 0,
            deep: 0,
            c: 0,
            d: 0,
        },
        tail: Tail {
            x: 0,
            y: 0,
        },
    };

    o.inner.deep = 0x11223344;

    if o.inner.x!= 0x11223344 {
        return 20;
    }

    o.inner.x = 5;
    o.inner.y = 6;
    if o.inner.x!= 5 {
        return 21;
    }
    if o.inner.y!= 6 {
        return 22;
    }

    o.tail.p = 40;
    o.tail.q = 41;
    if o.tail.p!= 40 {
        return 23;
    }
    if o.tail.q!= 41 {
        return 24;
    }

    o.tail.y = -9;
    if o.tail.y!= -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer {
        base: 0,
        inner: Inner {
            a: 0,
            b: 0,
            deep: 0,
            c: 0,
            d: 0,
        },
        tail: Tail {
            x: 0,
            y: 0,
        },
    };

    let pa = &o.inner.a as *const _ as usize;
    let pb = &o.inner.b as *const _ as usize;
    let pdeep = &o.inner.deep as *const _ as usize;
    let ptx = &o.tail.x as *const _ as usize;
    let pty = &o.tail.y as *const _ as usize;

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

#[derive(Debug)]
struct Inner {
    a: i32,
    b: i32,
    deep: i32,
    c: i32,
    d: i32,
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Tail {
    x: i32,
    y: i32,
    p: i32,
    q: i32,
}

#[derive(Debug)]
struct Outer {
    base: i32,
    inner: Inner,
    tail: Tail,
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