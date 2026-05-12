type uptr = usize;

#[repr(C)]
union DeepUnion {
    deep: i32,
    xy: XY,
}

#[repr(C)]
struct XY {
    x: i32,
    y: i32,
}

#[repr(C)]
struct InnerStruct2 {
    b: i32,
    deep_union: DeepUnion,
    c: i32,
}

#[repr(C)]
union InnerUnion {
    u_as_int: i32,
    inner_struct: InnerStruct2,
    u_as_unsigned: u32,
}

#[repr(C)]
struct InnerStruct {
    a: i32,
    inner_union: InnerUnion,
    d: i32,
}

#[repr(C)]
union TailUnion2 {
    tail_y: i32,
    pq: PQ,
}

#[repr(C)]
struct PQ {
    tail_p: i32,
    tail_q: i32,
}

#[repr(C)]
struct TailStruct {
    tail_x: i32,
    tail_union: TailUnion2,
}

#[repr(C)]
union TailUnion {
    tail_i: i32,
    tail_struct: TailStruct,
}

#[repr(C)]
struct Outer {
    base: i32,
    inner: InnerStruct,
    tail: TailUnion,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        inner: InnerStruct {
            a: 1,
            inner_union: InnerUnion {
                inner_struct: InnerStruct2 {
                    b: 2,
                    deep_union: DeepUnion { deep: 99 },
                    c: 3,
                }
            },
            d: 4,
        },
        tail: TailUnion {
            tail_struct: TailStruct {
                tail_x: 7,
                tail_union: TailUnion2 { tail_y: 8 },
            }
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.inner.a != 1 {
        return 2;
    }
    if unsafe { o.inner.inner_union.inner_struct.b } != 2 {
        return 3;
    }
    if unsafe { o.inner.inner_union.inner_struct.deep_union.deep } != 99 {
        return 4;
    }
    if unsafe { o.inner.inner_union.inner_struct.c } != 3 {
        return 5;
    }
    if o.inner.d != 4 {
        return 6;
    }
    if unsafe { o.tail.tail_struct.tail_x } != 7 {
        return 7;
    }
    if unsafe { o.tail.tail_struct.tail_union.tail_y } != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o: Outer = unsafe { std::mem::zeroed() };
    o.base = 0;

    unsafe { o.inner.inner_union.inner_struct.deep_union.deep = 0x11223344; }

    if unsafe { o.inner.inner_union.inner_struct.deep_union.xy.x } != 0x11223344 {
        return 20;
    }

    unsafe { o.inner.inner_union.inner_struct.deep_union.xy.x = 5; }
    unsafe { o.inner.inner_union.inner_struct.deep_union.xy.y = 6; }
    if unsafe { o.inner.inner_union.inner_struct.deep_union.xy.x } != 5 {
        return 21;
    }
    if unsafe { o.inner.inner_union.inner_struct.deep_union.xy.y } != 6 {
        return 22;
    }

    unsafe { o.tail.tail_struct.tail_union.pq.tail_p = 40; }
    unsafe { o.tail.tail_struct.tail_union.pq.tail_q = 41; }
    if unsafe { o.tail.tail_struct.tail_union.pq.tail_p } != 40 {
        return 23;
    }
    if unsafe { o.tail.tail_struct.tail_union.pq.tail_q } != 41 {
        return 24;
    }

    unsafe { o.tail.tail_struct.tail_union.tail_y = -9; }
    if unsafe { o.tail.tail_struct.tail_union.tail_y } != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o: Outer = unsafe { std::mem::zeroed() };

    let pa = &o.inner.a as *const i32;
    let pb = unsafe { &o.inner.inner_union.inner_struct.b as *const i32 };
    let pdeep = unsafe { &o.inner.inner_union.inner_struct.deep_union.deep as *const i32 };
    let ptx = unsafe { &o.tail.tail_struct.tail_x as *const i32 };
    let pty = unsafe { &o.tail.tail_struct.tail_union.tail_y as *const i32 };

    if pa.is_null() || pb.is_null() || pdeep.is_null() || ptx.is_null() || pty.is_null() {
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

    std::process::exit(0);
}