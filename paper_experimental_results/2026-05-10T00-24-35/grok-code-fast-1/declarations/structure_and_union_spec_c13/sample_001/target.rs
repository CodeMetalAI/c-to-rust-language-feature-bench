#[repr(C)]
#[derive(Clone, Copy)]
struct XY {
    x: i32,
    y: i32,
}

#[repr(C)]
union DeepUnion {
    deep: i32,
    xy: XY,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct InnerStruct {
    b: i32,
    deep_union: DeepUnion,
    c: i32,
}

#[repr(C)]
union MainUnion {
    u_as_int: i32,
    inner_struct: InnerStruct,
    u_as_unsigned: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Inner {
    a: i32,
    main_union: MainUnion,
    d: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TailPQ {
    tail_p: i32,
    tail_q: i32,
}

#[repr(C)]
union TailUnion {
    tail_y: i32,
    tail_pq: TailPQ,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TailStruct {
    tail_x: i32,
    tail_union: TailUnion,
}

#[repr(C)]
union TailUnionOuter {
    tail_i: i32,
    tail_struct: TailStruct,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Outer {
    base: i32,
    inner: Inner,
    tail: TailUnionOuter,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        inner: Inner {
            a: 1,
            main_union: MainUnion {
                inner_struct: InnerStruct {
                    b: 2,
                    deep_union: DeepUnion { deep: 99 },
                    c: 3,
                },
            },
            d: 4,
        },
        tail: TailUnionOuter {
            tail_struct: TailStruct {
                tail_x: 7,
                tail_union: TailUnion { tail_y: 8 },
            },
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.inner.a != 1 {
        return 2;
    }
    if unsafe { o.inner.main_union.inner_struct.b } != 2 {
        return 3;
    }
    if unsafe { o.inner.main_union.inner_struct.deep_union.deep } != 99 {
        return 4;
    }
    if unsafe { o.inner.main_union.inner_struct.c } != 3 {
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

    unsafe { o.inner.main_union.inner_struct.deep_union.deep = 0x11223344 };

    if unsafe { o.inner.main_union.inner_struct.deep_union.xy.x } != 0x11223344 {
        return 20;
    }

    unsafe { o.inner.main_union.inner_struct.deep_union.xy.x = 5 };
    unsafe { o.inner.main_union.inner_struct.deep_union.xy.y = 6 };
    if unsafe { o.inner.main_union.inner_struct.deep_union.xy.x } != 5 {
        return 21;
    }
    if unsafe { o.inner.main_union.inner_struct.deep_union.xy.y } != 6 {
        return 22;
    }

    unsafe { o.tail.tail_struct.tail_union.tail_pq.tail_p = 40 };
    unsafe { o.tail.tail_struct.tail_union.tail_pq.tail_q = 41 };
    if unsafe { o.tail.tail_struct.tail_union.tail_pq.tail_p } != 40 {
        return 23;
    }
    if unsafe { o.tail.tail_struct.tail_union.tail_pq.tail_q } != 41 {
        return 24;
    }

    unsafe { o.tail.tail_struct.tail_union.tail_y = -9 };
    if unsafe { o.tail.tail_struct.tail_union.tail_y } != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o: Outer = unsafe { std::mem::zeroed() };

    let pa = unsafe { &o.inner.a as *const _ as usize };
    let pb = unsafe { &o.inner.main_union.inner_struct.b as *const _ as usize };
    let pdeep = unsafe { &o.inner.main_union.inner_struct.deep_union.deep as *const _ as usize };
    let ptx = unsafe { &o.tail.tail_struct.tail_x as *const _ as usize };
    let pty = unsafe { &o.tail.tail_struct.tail_union.tail_y as *const _ as usize };

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
        std::process::exit(r as i32);
    }

    r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        std::process::exit(r as i32);
    }

    r = check_addressability();
    if r != 0 {
        std::process::exit(r as i32);
    }

    std::process::exit(0);
}