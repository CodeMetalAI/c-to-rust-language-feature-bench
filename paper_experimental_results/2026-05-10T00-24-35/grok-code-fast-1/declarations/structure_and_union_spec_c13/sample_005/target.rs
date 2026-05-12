#[repr(C)]
union Union1 {
    u_as_int: i32,
    inner_struct: Inner1,
    u_as_unsigned: u32,
}

#[repr(C)]
struct Inner1 {
    b: i32,
    deep_union: DeepUnion,
    c: i32,
}

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
union TailUnion {
    tail_i: i32,
    tail_struct: TailStruct,
}

#[repr(C)]
struct TailStruct {
    tail_x: i32,
    tail_subunion: TailSubUnion,
}

#[repr(C)]
union TailSubUnion {
    tail_y: i32,
    pq: PQ,
}

#[repr(C)]
struct PQ {
    tail_p: i32,
    tail_q: i32,
}

#[repr(C)]
struct Outer {
    base: i32,
    a: i32,
    union_part: Union1,
    d: i32,
    tail_union: TailUnion,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        a: 1,
        union_part: Union1 {
            inner_struct: Inner1 {
                b: 2,
                deep_union: DeepUnion { deep: 99 },
                c: 3,
            },
        },
        d: 4,
        tail_union: TailUnion {
            tail_struct: TailStruct {
                tail_x: 7,
                tail_subunion: TailSubUnion { tail_y: 8 },
            },
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.a != 1 {
        return 2;
    }
    if unsafe { o.union_part.inner_struct.b } != 2 {
        return 3;
    }
    if unsafe { o.union_part.inner_struct.deep_union.deep } != 99 {
        return 4;
    }
    if unsafe { o.union_part.inner_struct.c } != 3 {
        return 5;
    }
    if o.d != 4 {
        return 6;
    }
    if unsafe { o.tail_union.tail_struct.tail_x } != 7 {
        return 7;
    }
    if unsafe { o.tail_union.tail_struct.tail_subunion.tail_y } != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o: Outer = Outer {
        base: 0,
        a: 0,
        union_part: Union1 {
            inner_struct: Inner1 {
                b: 0,
                deep_union: DeepUnion { deep: 0 },
                c: 0,
            },
        },
        d: 0,
        tail_union: TailUnion {
            tail_struct: TailStruct {
                tail_x: 0,
                tail_subunion: TailSubUnion { tail_y: 0 },
            },
        },
    };

    o.base = 0;

    unsafe { o.union_part.inner_struct.deep_union.deep = 0x11223344 };

    if unsafe { o.union_part.inner_struct.deep_union.xy.x } != 0x11223344 {
        return 20;
    }

    unsafe { o.union_part.inner_struct.deep_union.xy.x = 5 };
    unsafe { o.union_part.inner_struct.deep_union.xy.y = 6 };
    if unsafe { o.union_part.inner_struct.deep_union.xy.x } != 5 {
        return 21;
    }
    if unsafe { o.union_part.inner_struct.deep_union.xy.y } != 6 {
        return 22;
    }

    unsafe { o.tail_union.tail_struct.tail_subunion.pq.tail_p = 40 };
    unsafe { o.tail_union.tail_struct.tail_subunion.pq.tail_q = 41 };
    if unsafe { o.tail_union.tail_struct.tail_subunion.pq.tail_p } != 40 {
        return 23;
    }
    if unsafe { o.tail_union.tail_struct.tail_subunion.pq.tail_q } != 41 {
        return 24;
    }

    unsafe { o.tail_union.tail_struct.tail_subunion.tail_y = -9 };
    if unsafe { o.tail_union.tail_struct.tail_subunion.tail_y } != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o: Outer = Outer {
        base: 0,
        a: 0,
        union_part: Union1 {
            inner_struct: Inner1 {
                b: 0,
                deep_union: DeepUnion { deep: 0 },
                c: 0,
            },
        },
        d: 0,
        tail_union: TailUnion {
            tail_struct: TailStruct {
                tail_x: 0,
                tail_subunion: TailSubUnion { tail_y: 0 },
            },
        },
    };

    let pa = std::ptr::from_ref(&o.a).addr();
    let pb = unsafe { std::ptr::from_ref(&o.union_part.inner_struct.b).addr() };
    let pdeep = unsafe { std::ptr::from_ref(&o.union_part.inner_struct.deep_union.deep).addr() };
    let ptx = unsafe { std::ptr::from_ref(&o.tail_union.tail_struct.tail_x).addr() };
    let pty = unsafe { std::ptr::from_ref(&o.tail_union.tail_struct.tail_subunion.tail_y).addr() };

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