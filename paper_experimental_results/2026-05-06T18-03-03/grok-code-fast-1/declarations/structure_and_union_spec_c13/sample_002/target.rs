#[repr(C)]
struct Outer {
    base: i32,
    inner: Inner,
    tail: Tail,
}

#[repr(C)]
struct Inner {
    a: i32,
    u: U,
    d: i32,
}

#[repr(C)]
union U {
    u_as_int: i32,
    inner_u: InnerU,
    u_as_unsigned: u32,
}

#[repr(C)]
struct InnerU {
    b: i32,
    deep_u: DeepU,
    c: i32,
}

#[repr(C)]
union DeepU {
    deep: i32,
    xy: XY,
}

#[repr(C)]
struct XY {
    x: i32,
    y: i32,
}

#[repr(C)]
union Tail {
    tail_i: i32,
    tail_struct: TailStruct,
}

#[repr(C)]
struct TailStruct {
    tail_x: i32,
    tail_u: TailU,
}

#[repr(C)]
union TailU {
    tail_y: i32,
    pq: PQ,
}

#[repr(C)]
struct PQ {
    tail_p: i32,
    tail_q: i32,
}

type Uptr = usize;

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        inner: Inner {
            a: 1,
            u: U {
                inner_u: InnerU {
                    b: 2,
                    deep_u: DeepU { deep: 99 },
                    c: 3,
                },
            },
            d: 4,
        },
        tail: Tail {
            tail_struct: TailStruct {
                tail_x: 7,
                tail_u: TailU { tail_y: 8 },
            },
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.inner.a != 1 {
        return 2;
    }
    if unsafe { o.inner.u.inner_u.b } != 2 {
        return 3;
    }
    if unsafe { o.inner.u.inner_u.deep_u.deep } != 99 {
        return 4;
    }
    if unsafe { o.inner.u.inner_u.c } != 3 {
        return 5;
    }
    if o.inner.d != 4 {
        return 6;
    }
    if unsafe { o.tail.tail_struct.tail_x } != 7 {
        return 7;
    }
    if unsafe { o.tail.tail_struct.tail_u.tail_y } != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o: Outer = unsafe { std::mem::zeroed() };
    o.base = 0;

    unsafe { o.inner.u.inner_u.deep_u.deep = 0x11223344 };

    if unsafe { o.inner.u.inner_u.deep_u.xy.x } != 0x11223344 {
        return 20;
    }

    unsafe { o.inner.u.inner_u.deep_u.xy.x = 5 };
    unsafe { o.inner.u.inner_u.deep_u.xy.y = 6 };
    if unsafe { o.inner.u.inner_u.deep_u.xy.x } != 5 {
        return 21;
    }
    if unsafe { o.inner.u.inner_u.deep_u.xy.y } != 6 {
        return 22;
    }

    unsafe { o.tail.tail_struct.tail_u.pq.tail_p = 40 };
    unsafe { o.tail.tail_struct.tail_u.pq.tail_q = 41 };
    if unsafe { o.tail.tail_struct.tail_u.pq.tail_p } != 40 {
        return 23;
    }
    if unsafe { o.tail.tail_struct.tail_u.pq.tail_q } != 41 {
        return 24;
    }

    unsafe { o.tail.tail_struct.tail_u.tail_y = -9 };
    if unsafe { o.tail.tail_struct.tail_u.tail_y } != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o: Outer = unsafe { std::mem::zeroed() };

    let pa: Uptr = unsafe { &o.inner.a as *const i32 as Uptr };
    let pb: Uptr = unsafe { &o.inner.u.inner_u.b as *const i32 as Uptr };
    let pdeep: Uptr = unsafe { &o.inner.u.inner_u.deep_u.deep as *const i32 as Uptr };
    let ptx: Uptr = unsafe { &o.tail.tail_struct.tail_x as *const i32 as Uptr };
    let pty: Uptr = unsafe { &o.tail.tail_struct.tail_u.tail_y as *const i32 as Uptr };

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
    let mut r = check_designated_init();
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