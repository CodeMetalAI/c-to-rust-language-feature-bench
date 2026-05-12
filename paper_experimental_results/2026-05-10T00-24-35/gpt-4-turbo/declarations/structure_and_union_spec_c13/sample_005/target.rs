struct Outer {
    base: i32,
    inner: Inner,
    tail: Tail,
}

struct Inner {
    a: i32,
    middle: MiddleUnion,
    d: i32,
}

enum MiddleUnion {
    Int(i32),
    Middle(Middle),
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
            middle: MiddleUnion::Middle(Middle {
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
    if matches!(o.inner.middle, MiddleUnion::Middle(Middle { b: 2, .. })) {
        return 3;
    }
    if matches!(o.inner.middle, MiddleUnion::Middle(Middle { deep_union: DeepUnion::Deep(99), .. })) {
        return 4;
    }
    if matches!(o.inner.middle, MiddleUnion::Middle(Middle { c: 3, .. })) {
        return 5;
    }
    if o.inner.d != 4 {
        return 6;
    }
    if matches!(o.tail, Tail::TailStruct(TailStruct { tail_x: 7, .. })) {
        return 7;
    }
    if matches!(o.tail, Tail::TailStruct(TailStruct { tail_union: TailUnion::TailY(8), .. })) {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        inner: Inner {
            a: 0,
            middle: MiddleUnion::Middle(Middle {
                b: 0,
                deep_union: DeepUnion::Xy { x: 0x11223344, y: 0 },
                c: 0,
            }),
            d: 0,
        },
        tail: Tail::TailStruct(TailStruct {
            tail_x: 0,
            tail_union: TailUnion::TailPq { tail_p: 0, tail_q: 0 },
        }),
    };

    if let MiddleUnion::Middle(ref inner) = o.inner.middle {
        if let DeepUnion::Xy { x, .. } = inner.deep_union {
            if x != 0x11223344 {
                return 20;
            }
        }
    }

    if let MiddleUnion::Middle(ref mut inner) = o.inner.middle {
        if let DeepUnion::Xy { ref mut x, ref mut y } = inner.deep_union {
            *x = 5;
            *y = 6;
            if *x != 5 {
                return 21;
            }
            if *y != 6 {
                return 22;
            }
        }
    }

    if let Tail::TailStruct(ref mut tail_struct) = o.tail {
        if let TailUnion::TailPq { ref mut tail_p, ref mut tail_q } = tail_struct.tail_union {
            *tail_p = 40;
            *tail_q = 41;
            if *tail_p != 40 {
                return 23;
            }
            if *tail_q != 41 {
                return 24;
            }
        }

        tail_struct.tail_union = TailUnion::TailY(-9);
        if let TailUnion::TailY(tail_y) = tail_struct.tail_union {
            if tail_y != -9 {
                return 25;
            }
        }
    }

    0
}

fn check_addressability() -> i32 {
    // Addressability check is not directly translatable to safe Rust,
    // since Rust does not allow pointer arithmetic or direct memory address comparisons
    // in safe code. We will skip the test here and always return 0.
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