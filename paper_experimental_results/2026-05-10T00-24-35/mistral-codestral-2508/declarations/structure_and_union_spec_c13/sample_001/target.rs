#[derive(Debug, Default)]
struct Deep {
    deep: i32,
    x: i32,
    y: i32,
}

#[derive(Debug, Default)]
struct B {
    b: i32,
    deep: Deep,
    c: i32,
}

#[derive(Debug, Default)]
struct InnerUnion {
    u_as_int: i32,
    b: B,
    u_as_unsigned: u32,
}

#[derive(Debug, Default)]
struct Inner {
    a: i32,
    inner_union: InnerUnion,
    d: i32,
}

#[derive(Debug, Default)]
struct TailUnion {
    tail_i: i32,
    tail_x: i32,
    tail_y: i32,
    tail_p: i32,
    tail_q: i32,
}

#[derive(Debug, Default)]
struct Outer {
    base: i32,
    inner: Inner,
    tail_union: TailUnion,
}

fn check_designated_init() -> i32 {
    let mut o = Outer {
        base: 10,
        inner: Inner {
            a: 1,
            inner_union: InnerUnion {
                u_as_int: 0,
                b: B {
                    b: 2,
                    deep: Deep {
                        deep: 99,
                        x: 0,
                        y: 0,
                    },
                    c: 3,
                },
                u_as_unsigned: 0,
            },
            d: 4,
        },
        tail_union: TailUnion {
            tail_i: 0,
            tail_x: 7,
            tail_y: 8,
            tail_p: 0,
            tail_q: 0,
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.inner.a != 1 {
        return 2;
    }
    if o.inner.inner_union.b.b != 2 {
        return 3;
    }
    if o.inner.inner_union.b.deep.deep != 99 {
        return 4;
    }
    if o.inner.inner_union.b.c != 3 {
        return 5;
    }
    if o.inner.d != 4 {
        return 6;
    }
    if o.tail_union.tail_x != 7 {
        return 7;
    }
    if o.tail_union.tail_y != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer::default();
    o.base = 0;

    o.inner.inner_union.b.deep.deep = 0x11223344;

    if o.inner.inner_union.b.deep.deep != 0x11223344 {
        return 20;
    }

    o.inner.inner_union.b.deep.x = 5;
    o.inner.inner_union.b.deep.y = 6;
    if o.inner.inner_union.b.deep.x != 5 {
        return 21;
    }
    if o.inner.inner_union.b.deep.y != 6 {
        return 22;
    }

    o.tail_union.tail_p = 40;
    o.tail_union.tail_q = 41;
    if o.tail_union.tail_p != 40 {
        return 23;
    }
    if o.tail_union.tail_q != 41 {
        return 24;
    }

    o.tail_union.tail_y = -9;
    if o.tail_union.tail_y != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer::default();

    let pa = &o.inner.a as *const i32 as usize;
    let pb = &o.inner.inner_union.b.b as *const i32 as usize;
    let pdeep = &o.inner.inner_union.b.deep.deep as *const i32 as usize;
    let ptx = &o.tail_union.tail_x as *const i32 as usize;
    let pty = &o.tail_union.tail_y as *const i32 as usize;

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