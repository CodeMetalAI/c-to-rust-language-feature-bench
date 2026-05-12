struct Outer {
    base: i32,

    a: i32,
    inner: InnerUnion,

    d: i32,

    tail: TailUnion,
}

union InnerUnion {
    u_as_int: i32,
    inner_struct: InnerStruct,
    u_as_unsigned: u32,
}

struct InnerStruct {
    b: i32,
    deep_union: DeepUnion,
    c: i32,
}

union DeepUnion {
    deep: i32,
    xy: XY,
}

struct XY {
    x: i32,
    y: i32,
}

union TailUnion {
    tail_i: i32,
    tail_struct: TailStruct,
}

struct TailStruct {
    tail_x: i32,
    tail_union: TailUnionInner,
}

union TailUnionInner {
    tail_y: i32,
    tail_pq: TailPQ,
}

struct TailPQ {
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        a: 1,
        inner: InnerUnion {
            inner_struct: InnerStruct {
                b: 2,
                deep_union: DeepUnion { deep: 99 },
                c: 3,
            },
        },
        d: 4,
        tail: TailUnion {
            tail_struct: TailStruct {
                tail_x: 7,
                tail_union: TailUnionInner { tail_y: 8 },
            },
        },
    };

    unsafe {
        if o.base != 10 {
            return 1;
        }
        if o.a != 1 {
            return 2;
        }
        if o.inner.inner_struct.b != 2 {
            return 3;
        }
        if o.inner.inner_struct.deep_union.deep != 99 {
            return 4;
        }
        if o.inner.inner_struct.c != 3 {
            return 5;
        }
        if o.d != 4 {
            return 6;
        }
        if o.tail.tail_struct.tail_x != 7 {
            return 7;
        }
        if o.tail.tail_struct.tail_union.tail_y != 8 {
            return 8;
        }
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        a: 0,
        inner: InnerUnion {
            inner_struct: InnerStruct {
                b: 0,
                deep_union: DeepUnion {
                    xy: XY { x: 0, y: 0 },
                },
                c: 0,
            },
        },
        d: 0,
        tail: TailUnion {
            tail_struct: TailStruct {
                tail_x: 0,
                tail_union: TailUnionInner {
                    tail_pq: TailPQ { tail_p: 0, tail_q: 0 },
                },
            },
        },
    };

    unsafe {
        o.inner.inner_struct.deep_union.xy.x = 0x11223344;

        if o.inner.inner_struct.deep_union.xy.x != 0x11223344 {
            return 20;
        }

        o.inner.inner_struct.deep_union.xy.x = 5;
        o.inner.inner_struct.deep_union.xy.y = 6;
        if o.inner.inner_struct.deep_union.xy.x != 5 {
            return 21;
        }
        if o.inner.inner_struct.deep_union.xy.y != 6 {
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
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer {
        base: 0,
        a: 0,
        inner: InnerUnion {
            inner_struct: InnerStruct {
                b: 0,
                deep_union: DeepUnion {
                    xy: XY { x: 0, y: 0 },
                },
                c: 0,
            },
        },
        d: 0,
        tail: TailUnion {
            tail_struct: TailStruct {
                tail_x: 0,
                tail_union: TailUnionInner {
                    tail_pq: TailPQ { tail_p: 0, tail_q: 0 },
                },
            },
        },
    };

    let pa = &o.a as *const _ as usize;
    let pb = unsafe { &o.inner.inner_struct.b } as *const _ as usize;
    let pdeep = unsafe { &o.inner.inner_struct.deep_union.deep } as *const _ as usize;
    let ptx = unsafe { &o.tail.tail_struct.tail_x } as *const _ as usize;
    let pty = unsafe { &o.tail.tail_struct.tail_union.tail_y } as *const _ as usize;

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