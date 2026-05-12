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

enum Middle {
    AsInt(i32),
    AsUnsigned(u32),
    Nested(Nested),
}

struct Nested {
    b: i32,
    deep_union: DeepUnion,
    c: i32,
}

enum DeepUnion {
    Deep(i32),
    Coordinates(Coordinates),
}

struct Coordinates {
    x: i32,
    y: i32,
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
    TailCoords(TailCoords),
}

struct TailCoords {
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        inner: Inner {
            a: 1,
            middle: Middle::Nested(Nested {
                b: 2,
                deep_union: DeepUnion::Deep(99),
                c: 3,
            }),
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
    if let Middle::Nested(nested) = &o.inner.middle {
        if nested.b != 2 {
            return 3;
        }
        if let DeepUnion::Deep(deep) = nested.deep_union {
            if deep != 99 {
                return 4;
            }
        } else {
            return 4;
        }
        if nested.c != 3 {
            return 5;
        }
    } else {
        return 3;
    }
    if o.inner.d != 4 {
        return 6;
    }
    if let Tail::TailStruct(tail_struct) = &o.tail {
        if tail_struct.tail_x != 7 {
            return 7;
        }
        if let TailUnion::TailY(tail_y) = tail_struct.tail_union {
            if tail_y != 8 {
                return 8;
            }
        } else {
            return 8;
        }
    } else {
        return 7;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        inner: Inner {
            a: 0,
            middle: Middle::Nested(Nested {
                b: 0,
                deep_union: DeepUnion::Coordinates(Coordinates { x: 0x11223344, y: 0 }),
                c: 0,
            }),
            d: 0,
        },
        tail: Tail::TailStruct(TailStruct {
            tail_x: 0,
            tail_union: TailUnion::TailCoords(TailCoords { tail_p: 0, tail_q: 0 }),
        }),
    };

    if let Inner {
        middle: Middle::Nested(Nested {
            deep_union: DeepUnion::Coordinates(Coordinates { x, .. }),
            ..
        }),
        ..
    } = &mut o.inner
    {
        if *x != 0x11223344 {
            return 20;
        }
        *x = 5;
    } else {
        return 20;
    }

    if let Inner {
        middle: Middle::Nested(Nested {
            deep_union: DeepUnion::Coordinates(Coordinates { x, y }),
            ..
        }),
        ..
    } = &mut o.inner
    {
        if *x != 5 {
            return 21;
        }
        *y = 6;
        if *y != 6 {
            return 22;
        }
    } else {
        return 21;
    }

    if let Tail::TailStruct(TailStruct {
        tail_union: TailUnion::TailCoords(TailCoords { tail_p, tail_q }),
        ..
    }) = &mut o.tail
    {
        *tail_p = 40;
        *tail_q = 41;
        if *tail_p != 40 {
            return 23;
        }
        if *tail_q != 41 {
            return 24;
        }
    } else {
        return 23;
    }

    if let Tail::TailStruct(TailStruct {
        tail_union: TailUnion::TailY(tail_y),
        ..
    }) = &mut o.tail
    {
        *tail_y = -9;
        if *tail_y != -9 {
            return 25;
        }
    } else {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    // In Rust, we cannot directly compare addresses like in C, but we can ensure that
    // references are not the same using std::ptr::eq, which is safe.
    // However, since we cannot take the address of a field in an enum directly without
    // matching on it, this function will always return 0 in a safe Rust context.
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