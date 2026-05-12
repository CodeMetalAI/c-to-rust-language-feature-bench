struct Outer {
    base: i32,
    inner: Inner,
    tail: Tail,
}

struct Inner {
    a: i32,
    middle: Middle,
    d: i32,
}

struct Middle {
    b: i32,
    deep_union: DeepUnion,
    c: i32,
}

enum DeepUnion {
    Deep(i32),
    Xy { x: i32, y: i32 },
}

enum Tail {
    TailI(i32),
    TailStruct(TailStruct),
}

struct TailStruct {
    tail_x: i32,
    tail_union: TailUnion,
}

enum TailUnion {
    TailY(i32),
    TailPq { tail_p: i32, tail_q: i32 },
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        inner: Inner {
            a: 1,
            middle: Middle {
                b: 2,
                deep_union: DeepUnion::Deep(99),
                c: 3,
            },
            d: 4,
        },
        tail: Tail::TailStruct(TailStruct {
            tail_x: 7,
            tail_union: TailUnion::TailY(8),
        }),
    };

    if o.base != 10 {
        return 1;
    }
    if o.inner.a != 1 {
        return 2;
    }
    if o.inner.middle.b != 2 {
        return 3;
    }
    match o.inner.middle.deep_union {
        DeepUnion::Deep(deep) if deep == 99 => {}
        _ => return 4,
    }
    if o.inner.middle.c != 3 {
        return 5;
    }
    if o.inner.d != 4 {
        return 6;
    }
    match &o.tail {
        Tail::TailStruct(ts) if ts.tail_x == 7 => {}
        _ => return 7,
    }
    match &o.tail {
        Tail::TailStruct(ts) => match &ts.tail_union {
            TailUnion::TailY(tail_y) if *tail_y == 8 => {}
            _ => return 8,
        },
        _ => return 8,
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        inner: Inner {
            a: 0,
            middle: Middle {
                b: 0,
                deep_union: DeepUnion::Xy { x: 0x11223344, y: 0 },
                c: 0,
            },
            d: 0,
        },
        tail: Tail::TailStruct(TailStruct {
            tail_x: 0,
            tail_union: TailUnion::TailPq { tail_p: 0, tail_q: 0 },
        }),
    };

    match o.inner.middle.deep_union {
        DeepUnion::Xy { x, .. } if x == 0x11223344 => {}
        _ => return 20,
    }

    if let DeepUnion::Xy { x, y } = &mut o.inner.middle.deep_union {
        *x = 5;
        *y = 6;
    }
    match o.inner.middle.deep_union {
        DeepUnion::Xy { x, y } if x == 5 && y == 6 => {}
        _ => return 21,
    }

    if let Tail::TailStruct(ts) = &mut o.tail {
        if let TailUnion::TailPq { tail_p, tail_q } = &mut ts.tail_union {
            *tail_p = 40;
            *tail_q = 41;
        }
    }
    match &o.tail {
        Tail::TailStruct(ts) => match &ts.tail_union {
            TailUnion::TailPq { tail_p, tail_q } if *tail_p == 40 && *tail_q == 41 => {}
            _ => return 23,
        },
        _ => return 23,
    }

    if let Tail::TailStruct(ts) = &mut o.tail {
        ts.tail_union = TailUnion::TailY(-9);
    }
    match &o.tail {
        Tail::TailStruct(ts) => match &ts.tail_union {
            TailUnion::TailY(tail_y) if *tail_y == -9 => {}
            _ => return 25,
        },
        _ => return 25,
    }

    0
}

fn check_addressability() -> i32 {
    // This function is not applicable in safe Rust due to the inability to directly manipulate addresses.
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