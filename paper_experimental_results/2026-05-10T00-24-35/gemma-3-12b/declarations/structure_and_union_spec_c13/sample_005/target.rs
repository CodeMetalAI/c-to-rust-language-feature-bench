#[repr(align(4))]
struct Outer {
    base: i32,
    a: NestedOuter,
    tail: TailOuter,
}

#[repr(align(4))]
struct NestedOuter {
    a: i32,
    inner: InnerOuter,
    d: i32,
}

#[repr(align(4))]
struct InnerOuter {
    u: UnionInner,
}

#[repr(align(4))]
union UnionInner {
    u_as_int: i32,
    u_as_unsigned: u32,
    inner: InnerNested,
}

#[repr(align(4))]
struct InnerNested {
    b: i32,
    innermost: Innermost,
    c: i32,
}

#[repr(align(4))]
union Innermost {
    deep: i32,
    fields: FieldPair,
}

#[repr(align(4))]
struct FieldPair {
    x: i32,
    y: i32,
}

#[repr(align(4))]
struct TailOuter {
    tail_x: i32,
    tail_union: TailUnion,
}

#[repr(align(4))]
union TailUnion {
    tail_y: i32,
    tail_fields: TailFieldPair,
}

#[repr(align(4))]
struct TailFieldPair {
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        a: NestedOuter {
            a: 1,
            inner: InnerOuter {
                u: UnionInner {
                    inner: InnerNested {
                        b: 2,
                        innermost: Innermost {
                            deep: 99,
                        },
                        c: 3,
                    },
                },
            },
            d: 4,
        },
        tail: TailOuter {
            tail_x: 7,
            tail_union: TailUnion {
                tail_y: 8,
            },
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.a.a != 1 {
        return 2;
    }
    if o.a.inner.u.inner.b != 2 {
        return 3;
    }
    if o.a.inner.u.inner.innermost.deep != 99 {
        return 4;
    }
    if o.a.inner.u.inner.c != 3 {
        return 5;
    }
    if o.a.d != 4 {
        return 6;
    }
    if o.tail.tail_x != 7 {
        return 7;
    }
    if o.tail.tail_union.tail_y != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        a: NestedOuter {
            a: 0,
            inner: InnerOuter {
                u: UnionInner {
                    u_as_int: 0,
                    inner: InnerNested {
                        b: 0,
                        innermost: Innermost {
                            deep: 0,
                            fields: FieldPair { x: 0, y: 0 },
                        },
                        c: 0,
                    },
                },
            },
            d: 0,
        },
        tail: TailOuter {
            tail_x: 0,
            tail_union: TailUnion {
                tail_y: 0,
                tail_fields: TailFieldPair {
                    tail_p: 0,
                    tail_q: 0,
                },
            },
        },
    };

    o.a.inner.u.inner.innermost.deep = 0x11223344 as i32;

    if o.a.inner.u.inner.innermost.fields.x != 0x11223344 {
        return 20;
    }

    o.a.inner.u.inner.innermost.fields.x = 5;
    o.a.inner.u.inner.innermost.fields.y = 6;

    if o.a.inner.u.inner.innermost.fields.x != 5 {
        return 21;
    }
    if o.a.inner.u.inner.innermost.fields.y != 6 {
        return 22;
    }

    o.tail.tail_union.tail_fields.tail_p = 40;
    o.tail.tail_union.tail_fields.tail_q = 41;

    if o.tail.tail_union.tail_fields.tail_p != 40 {
        return 23;
    }
    if o.tail.tail_union.tail_fields.tail_q != 41 {
        return 24;
    }

    o.tail.tail_union.tail_y = -9;
    if o.tail.tail_union.tail_y != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer {
        base: 0,
        a: NestedOuter {
            a: 0,
            inner: InnerOuter {
                u: UnionInner {
                    u_as_int: 0,
                    inner: InnerNested {
                        b: 0,
                        innermost: Innermost {
                            deep: 0,
                            fields: FieldPair { x: 0, y: 0 },
                        },
                        c: 0,
                    },
                },
            },
            d: 0,
        },
        tail: TailOuter {
            tail_x: 0,
            tail_union: TailUnion {
                tail_y: 0,
                tail_fields: TailFieldPair {
                    tail_p: 0,
                    tail_q: 0,
                },
            },
        },
    };
    let _pa: *const i32 = &o.a.a;
    let _pb: *const i32 = &o.a.inner.u.inner.b;
    let _pdeep: *const i32 = &o.a.inner.u.inner.innermost.deep;
    let _ptx: *const i32 = &o.tail.tail_x;
    let _pty: *const i32 = &o.tail.tail_union.tail_y;

    if _pa.is_null() || _pb.is_null() || _pdeep.is_null() || _ptx.is_null() || _pty.is_null() {
        return 30;
    }

    if _pa == _pb {
        return 31;
    }
    if _ptx == _pty {
        return 32;
    }

    0
}

fn main() {
    let mut r = check_designated_init();
    if r {
        std::process::exit(r as i32);
    }

    r = check_union_aliasing_via_flattened_names();
    if r {
        std::process::exit(r as i32);
    }

    r = check_addressability();
    if r {
        std::process::exit(r as i32);
    }

    std::process::exit(0);
}