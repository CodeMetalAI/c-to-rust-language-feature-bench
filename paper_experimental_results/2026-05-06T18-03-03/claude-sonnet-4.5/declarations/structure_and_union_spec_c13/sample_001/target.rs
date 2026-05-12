use std::mem;

type Uptr = usize;

#[repr(C)]
struct Outer {
    base: i32,
    anonymous1: Anonymous1,
    anonymous2: Anonymous2,
}

#[repr(C)]
struct Anonymous1 {
    a: i32,
    anonymous1_union: Anonymous1Union,
    d: i32,
}

#[repr(C)]
union Anonymous1Union {
    u_as_int: i32,
    anonymous1_struct: Anonymous1Struct,
    u_as_unsigned: u32,
}

#[repr(C)]
struct Anonymous1Struct {
    b: i32,
    anonymous1_inner_union: Anonymous1InnerUnion,
    c: i32,
}

#[repr(C)]
union Anonymous1InnerUnion {
    deep: i32,
    anonymous1_inner_struct: Anonymous1InnerStruct,
}

#[repr(C)]
struct Anonymous1InnerStruct {
    x: i32,
    y: i32,
}

#[repr(C)]
union Anonymous2 {
    tail_i: i32,
    anonymous2_struct: Anonymous2Struct,
}

#[repr(C)]
struct Anonymous2Struct {
    tail_x: i32,
    anonymous2_union: Anonymous2Union,
}

#[repr(C)]
union Anonymous2Union {
    tail_y: i32,
    anonymous2_inner_struct: Anonymous2InnerStruct,
}

#[repr(C)]
struct Anonymous2InnerStruct {
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        anonymous1: Anonymous1 {
            a: 1,
            anonymous1_union: Anonymous1Union {
                anonymous1_struct: Anonymous1Struct {
                    b: 2,
                    anonymous1_inner_union: Anonymous1InnerUnion { deep: 99 },
                    c: 3,
                },
            },
            d: 4,
        },
        anonymous2: Anonymous2 {
            anonymous2_struct: Anonymous2Struct {
                tail_x: 7,
                anonymous2_union: Anonymous2Union { tail_y: 8 },
            },
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.anonymous1.a != 1 {
        return 2;
    }
    if unsafe { o.anonymous1.anonymous1_union.anonymous1_struct.b } != 2 {
        return 3;
    }
    if unsafe { o.anonymous1.anonymous1_union.anonymous1_struct.anonymous1_inner_union.deep } != 99 {
        return 4;
    }
    if unsafe { o.anonymous1.anonymous1_union.anonymous1_struct.c } != 3 {
        return 5;
    }
    if o.anonymous1.d != 4 {
        return 6;
    }
    if unsafe { o.anonymous2.anonymous2_struct.tail_x } != 7 {
        return 7;
    }
    if unsafe { o.anonymous2.anonymous2_struct.anonymous2_union.tail_y } != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o: Outer = unsafe { mem::zeroed() };
    o.base = 0;

    o.anonymous1.anonymous1_union.anonymous1_struct.anonymous1_inner_union.deep = 0x11223344;

    if unsafe { o.anonymous1.anonymous1_union.anonymous1_struct.anonymous1_inner_union.anonymous1_inner_struct.x } != 0x11223344 {
        return 20;
    }

    unsafe {
        o.anonymous1.anonymous1_union.anonymous1_struct.anonymous1_inner_union.anonymous1_inner_struct.x = 5;
        o.anonymous1.anonymous1_union.anonymous1_struct.anonymous1_inner_union.anonymous1_inner_struct.y = 6;
    }
    if unsafe { o.anonymous1.anonymous1_union.anonymous1_struct.anonymous1_inner_union.anonymous1_inner_struct.x } != 5 {
        return 21;
    }
    if unsafe { o.anonymous1.anonymous1_union.anonymous1_struct.anonymous1_inner_union.anonymous1_inner_struct.y } != 6 {
        return 22;
    }

    unsafe {
        o.anonymous2.anonymous2_struct.anonymous2_union.anonymous2_inner_struct.tail_p = 40;
        o.anonymous2.anonymous2_struct.anonymous2_union.anonymous2_inner_struct.tail_q = 41;
    }
    if unsafe { o.anonymous2.anonymous2_struct.anonymous2_union.anonymous2_inner_struct.tail_p } != 40 {
        return 23;
    }
    if unsafe { o.anonymous2.anonymous2_struct.anonymous2_union.anonymous2_inner_struct.tail_q } != 41 {
        return 24;
    }

    o.anonymous2.anonymous2_struct.anonymous2_union.tail_y = -9;
    if unsafe { o.anonymous2.anonymous2_struct.anonymous2_union.tail_y } != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let mut o: Outer = unsafe { mem::zeroed() };

    let pa = &o.anonymous1.a as *const i32 as Uptr;
    let pb = unsafe { &o.anonymous1.anonymous1_union.anonymous1_struct.b as *const i32 as Uptr };
    let pdeep = unsafe { &o.anonymous1.anonymous1_union.anonymous1_struct.anonymous1_inner_union.deep as *const i32 as Uptr };
    let ptx = unsafe { &o.anonymous2.anonymous2_struct.tail_x as *const i32 as Uptr };
    let pty = unsafe { &o.anonymous2.anonymous2_struct.anonymous2_union.tail_y as *const i32 as Uptr };

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

    std::process::exit(0);
}