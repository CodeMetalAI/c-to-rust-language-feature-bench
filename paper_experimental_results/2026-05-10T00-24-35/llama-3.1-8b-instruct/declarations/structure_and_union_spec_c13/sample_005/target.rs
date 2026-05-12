struct Outer {
    base: i32,
    inner: Inner,
    tail: Tail,
}

struct Inner {
    a: i32,
    deep: i32,
    b: i32,
    c: i32,
    d: i32,
}

struct Tail {
    tail_x: i32,
    tail_y: i32,
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        inner: Inner {
            a: 1,
            deep: 99,
            b: 2,
            c: 3,
            d: 4,
        },
        tail: Tail {
            tail_x: 7,
            tail_y: 8,
           ..Default::default()
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
        inner: Default::default(),
        tail: Default::default(),
    };

    o.deep = 0x11223344;

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

    o.tail.tail_p = 40;
    o.tail.tail_q = 41;
    if o.tail.tail_p!= 40 {
        return 23;
    }
    if o.tail.tail_q!= 41 {
        return 24;
    }

    o.tail.tail_y = -9;
    if o.tail.tail_y!= -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let mut o = Outer {
        base: 0,
        inner: Default::default(),
        tail: Default::default(),
    };

    let pa = o.inner.a as *const i32 as usize;
    let pb = o.inner.b as *const i32 as usize;
    let pdeep = o.inner.deep as *const i32 as usize;
    let ptx = o.tail.tail_x as *const i32 as usize;
    let pty = o.tail.tail_y as *const i32 as usize;

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
        println!("{}", r);
        std::process::exit(r);
    }

    r = check_union_aliasing_via_flattened_names();
    if r!= 0 {
        println!("{}", r);
        std::process::exit(r);
    }

    r = check_addressability();
    if r!= 0 {
        println!("{}", r);
        std::process::exit(r);
    }

    println!("0");
}