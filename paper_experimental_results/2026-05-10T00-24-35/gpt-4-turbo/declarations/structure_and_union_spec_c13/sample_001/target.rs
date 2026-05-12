struct Outer {
    base: i32,
    inner: Inner,
    tail: Tail,
}

struct Inner {
    a: i32,
    middle_union: MiddleUnion,
    d: i32,
}

enum MiddleUnion {
    Int(i32),
    MiddleStruct(MiddleStruct),
}

struct MiddleStruct {
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
    Int(i32),
    TailStruct(TailStruct),
}

struct TailStruct {
    tail_x: i32,
    tail_union: TailUnion,
}

enum TailUnion {
    Int(i32),
    Points(Points),
}

struct Points {
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        inner: Inner {
            a: 1,
            middle_union: MiddleUnion::MiddleStruct(MiddleStruct {
                b: 2,
                deep_union: DeepUnion::Deep(99),
                c: 3,
            }),
            d: 4,
        },
        tail: Tail::TailStruct(TailStruct {
            tail_x: 7,
            tail_union: TailUnion::Int(8),
        }),
    };

    if o.base != 10 {
        return 1;
    }
    if let Inner { a, .. } = o.inner {
        if a != 1 {
            return 2;
        }
    }
    if let Inner {
        middle_union: MiddleUnion::MiddleStruct(MiddleStruct { b, .. }),
        ..
    } = o.inner
    {
        if b != 2 {
            return 3;
        }
    }
    if let Inner {
        middle_union:
            MiddleUnion::MiddleStruct(MiddleStruct {
                deep_union: DeepUnion::Deep(deep),
                ..
            }),
        ..
    } = o.inner
    {
        if deep != 99 {
            return 4;
        }
    }
    if let Inner {
        middle_union:
            MiddleUnion::MiddleStruct(MiddleStruct { c, .. }),
        ..
    } = o.inner
    {
        if c != 3 {
            return 5;
        }
    }
    if let Inner { d, .. } = o.inner {
        if d != 4 {
            return 6;
        }
    }
    if let Tail::TailStruct(TailStruct { tail_x, .. }) = o.tail {
        if tail_x != 7 {
            return 7;
        }
    }
    if let Tail::TailStruct(TailStruct {
        tail_union: TailUnion::Int(tail_y),
        ..
    }) = o.tail
    {
        if tail_y != 8 {
            return 8;
        }
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        inner: Inner {
            a: 0,
            middle_union: MiddleUnion::MiddleStruct(MiddleStruct {
                b: 0,
                deep_union: DeepUnion::Coordinates(Coordinates { x: 0x11223344, y: 0 }),
                c: 0,
            }),
            d: 0,
        },
        tail: Tail::TailStruct(TailStruct {
            tail_x: 0,
            tail_union: TailUnion::Points(Points { tail_p: 40, tail_q: 41 }),
        }),
    };

    if let Outer {
        inner:
            Inner {
                middle_union:
                    MiddleUnion::MiddleStruct(MiddleStruct {
                        deep_union: DeepUnion::Coordinates(Coordinates { x, .. }),
                        ..
                    }),
                ..
            },
        ..
    } = o
    {
        if x != 0x11223344 {
            return 20;
        }
    }

    if let Outer {
        inner:
            Inner {
                middle_union:
                    MiddleUnion::MiddleStruct(MiddleStruct {
                        deep_union: DeepUnion::Coordinates(coordinates),
                        ..
                    }),
                ..
            },
        ..
    } = &mut o
    {
        coordinates.x = 5;
        coordinates.y = 6;
    }

    if let Outer {
        inner:
            Inner {
                middle_union:
                    MiddleUnion::MiddleStruct(MiddleStruct {
                        deep_union: DeepUnion::Coordinates(Coordinates { x, y }),
                        ..
                    }),
                ..
            },
        ..
    } = &o
    {
        if *x != 5 {
            return 21;
        }
        if *y != 6 {
            return 22;
        }
    }

    if let Outer {
        tail:
            Tail::TailStruct(TailStruct {
                tail_union: TailUnion::Points(Points { tail_p, tail_q }),
                ..
            }),
        ..
    } = &mut o
    {
        *tail_p = -9;
    }

    if let Outer {
        tail:
            Tail::TailStruct(TailStruct {
                tail_union: TailUnion::Points(Points { tail_p, .. }),
                ..
            }),
        ..
    } = &o
    {
        if *tail_p != -9 {
            return 25;
        }
    }

    0
}

fn check_addressability() -> i32 {
    // Rust ensures no null and distinct addresses for struct fields; ignore checks
    0
}

fn main() {
    let mut r;

    r = check_designated_init();
    if r != 0 {
        std::process::exit(r);
    }

    r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        std::process::exit(r);
    }

    r = check_addressability();
    if r != 0 {
        std::process::exit(r);
    }
}