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
    if let DeepUnion::Deep(deep) = o.inner.middle.deep_union {
        if deep != 99 {
            return 4;
        }
    } else {
        return 4;
    }
    if o.inner.middle.c != 3 {
        return 5;
    }
    if o.inner.d != 4 {
        return 6;
    }
    if let Tail::TailStruct(ts) = &o.tail {
        if ts.tail_x != 7 {
            return 7;
        }
        if let TailUnion::TailY(tail_y) = ts.tail_union {
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

    if let Inner {
        middle: Middle {
            deep_union: DeepUnion::Xy { x, .. },
            ..
        },
        ..
    } = &o.inner
    {
        if *x != 0x11223344 {
            return 20;
        }
    } else {
        return 20;
    }

    if let Inner {
        middle:
            Middle {
                deep_union: DeepUnion::Xy { x, y },
                ..
            },
        ..
    } = &mut o.inner
    {
        *x = 5;
        *y = 6;
        if *x != 5 {
            return 21;
        }
        if *y != 6 {
            return 22;
        }
    }

    if let Tail::TailStruct(TailStruct {
        tail_union: TailUnion::TailPq { tail_p, tail_q },
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
    }

    0
}

fn check_addressability() -> i32 {
    // In Rust, we cannot directly check addresses like in C, but we can ensure
    // that references are not the same by using `std::ptr::eq`.
    let mut o = Outer {
        base: 0,
        inner: Inner {
            a: 0,
            middle: Middle {
                b: 0,
                deep_union: DeepUnion::Deep(0),
                c: 0,
            },
            d: 0,
        },
        tail: Tail::TailStruct(TailStruct {
            tail_x: 0,
            tail_union: TailUnion::TailY(0),
        }),
    };

    let pa = &o.inner.a as *const i32 as usize;
    let pb = &o.inner.middle.b as *const i32 as usize;
    let pdeep = match &o.inner.middle.deep_union {
        DeepUnion::Deep(deep) => deep as *const i32 as usize,
        _ => 0,
    };
    let ptx = if let Tail::TailStruct(ts) = &o.tail {
        &ts.tail_x as *const i32 as usize
    } else {
        0
    };
    let pty = if let Tail::TailStruct(TailStruct {
        tail_union: TailUnion::TailY(tail_y),
        ..
    }) = &o.tail
    {
        tail_y as *const i32 as usize
    } else {
        0
    };

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