#[derive(Default)]
struct Outer {
    base: i32,
    inner: Inner,
    tail: Tail,
}

#[derive(Default)]
struct Inner {
    a: i32,
    union1: Union1,
    d: i32,
}

#[derive(Default)]
struct Union1 {
    u_as_int: i32,
    inner2: Inner2,
    u_as_unsigned: u32,
}

#[derive(Default)]
struct Inner2 {
    b: i32,
    union2: Union2,
    c: i32,
}

#[derive(Default)]
struct Union2 {
    deep: i32,
    xy: XY,
}

#[derive(Default)]
struct XY {
    x: i32,
    y: i32,
}

#[derive(Default)]
struct Tail {
    tail_i: i32,
    tail_struct: TailStruct,
}

#[derive(Default)]
struct TailStruct {
    tail_x: i32,
    tail_union: TailUnion,
}

#[derive(Default)]
struct TailUnion {
    tail_y: i32,
    tail_pq: TailPQ,
}

#[derive(Default)]
struct TailPQ {
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let mut o = Outer {
        base: 10,
        inner: Inner {
            a: 1,
            union1: Union1 {
                inner2: Inner2 {
                    b: 2,
                    union2: Union2 {
                        deep: 99,
                        ..Default::default()
                    },
                    c: 3,
                },
                ..Default::default()
            },
            d: 4,
        },
        tail: Tail {
            tail_struct: TailStruct {
                tail_x: 7,
                tail_union: TailUnion {
                    tail_y: 8,
                    ..Default::default()
                },
            },
            ..Default::default()
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.inner.a != 1 {
        return 2;
    }
    if o.inner.union1.inner2.b != 2 {
        return 3;
    }
    if o.inner.union1.inner2.union2.deep != 99 {
        return 4;
    }
    if o.inner.union1.inner2.c != 3 {
        return 5;
    }
    if o.inner.d != 4 {
        return 6;
    }
    if o.tail.tail_struct.tail_x != 7 {
        return 7;
    }
    if o.tail.tail_struct.tail_union.tail_y != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer::default();
    o.base = 0;

    o.inner.union1.inner2.union2.deep = 0x11223344;

    if o.inner.union1.inner2.union2.xy.x != 0x11223344 {
        return 20;
    }

    o.inner.union1.inner2.union2.xy.x = 5;
    o.inner.union1.inner2.union2.xy.y = 6;
    if o.inner.union1.inner2.union2.xy.x != 5 {
        return 21;
    }
    if o.inner.union1.inner2.union2.xy.y != 6 {
        return 22;
    }

    o.tail.tail_struct.tail_union.tail_pq.tail_p = 40;
    o.tail.tail_struct.tail_union.tail_pq.tail_q = 41;
    if o.tail.tail_struct.tail_union.tail_pq.tail_p != 40 {
        return 23;
    }
    if o.tail.tail_struct.tail_union.tail_pq.tail_q != 41 {
        return 24;
    }

    o.tail.tail_struct.tail_union.tail_y = -9;
    if o.tail.tail_struct.tail_union.tail_y != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer::default();

    let pa = &o.inner.a as *const _ as usize;
    let pb = &o.inner.union1.inner2.b as *const _ as usize;
    let pdeep = &o.inner.union1.inner2.union2.deep as *const _ as usize;
    let ptx = &o.tail.tail_struct.tail_x as *const _ as usize;
    let pty = &o.tail.tail_struct.tail_union.tail_y as *const _ as usize;

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

fn main() -> i32 {
    let mut r;

    r = check_designated_init();
    if r != 0 {
        return r;
    }

    r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        return r;
    }

    r = check_addressability();
    if r != 0 {
        return r;
    }

    0
}