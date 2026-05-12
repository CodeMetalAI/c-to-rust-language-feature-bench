use std::mem;
use std::process;

#[repr(C)]
struct Outer {
    base: i32,
    a: i32,
    u: U,
    d: i32,
    tail: Tail,
}

#[repr(C)]
union U {
    u_as_int: i32,
    s: S,
    u_as_unsigned: u32,
}

#[repr(C)]
struct S {
    b: i32,
    u_deep: U2,
    c: i32,
}

#[repr(C)]
union U2 {
    deep: i32,
    sxy: StructXY,
}

#[repr(C)]
struct StructXY {
    x: i32,
    y: i32,
}

#[repr(C)]
union Tail {
    tail_i: i32,
    ts: TS,
}

#[repr(C)]
struct TS {
    tail_x: i32,
    tail_u: TailU,
}

#[repr(C)]
union TailU {
    tail_y: i32,
    tspq: StructPQ,
}

#[repr(C)]
struct StructPQ {
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        a: 1,
        u: U {
            s: S {
                b: 2,
                u_deep: U2 { deep: 99 },
                c: 3,
            },
        },
        d: 4,
        tail: Tail {
            ts: TS {
                tail_x: 7,
                tail_u: TailU { tail_y: 8 },
            },
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.a != 1 {
        return 2;
    }
    if unsafe { o.u.s.b } != 2 {
        return 3;
    }
    if unsafe { o.u.s.u_deep.deep } != 99 {
        return 4;
    }
    if unsafe { o.u.s.c } != 3 {
        return 5;
    }
    if o.d != 4 {
        return 6;
    }
    if unsafe { o.tail.ts.tail_x } != 7 {
        return 7;
    }
    if unsafe { o.tail.ts.tail_u.tail_y } != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o: Outer = unsafe { mem::zeroed() };
    o.base = 0;

    unsafe {
        o.u.s.u_deep.deep = 0x11223344;
    }

    if unsafe { o.u.s.u_deep.sxy.x } != 0x11223344 {
        return 20;
    }

    unsafe {
        o.u.s.u_deep.sxy.x = 5;
        o.u.s.u_deep.sxy.y = 6;
    }
    if unsafe { o.u.s.u_deep.sxy.x } != 5 {
        return 21;
    }
    if unsafe { o.u.s.u_deep.sxy.y } != 6 {
        return 22;
    }

    unsafe {
        o.tail.ts.tail_u.tspq.tail_p = 40;
        o.tail.ts.tail_u.tspq.tail_q = 41;
    }
    if unsafe { o.tail.ts.tail_u.tspq.tail_p } != 40 {
        return 23;
    }
    if unsafe { o.tail.ts.tail_u.tspq.tail_q } != 41 {
        return 24;
    }

    unsafe {
        o.tail.ts.tail_u.tail_y = -9;
    }
    if unsafe { o.tail.ts.tail_u.tail_y } != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o: Outer = unsafe { mem::zeroed() };

    let pa = (&o.a as *const i32 as usize);
    let pb = unsafe { (&o.u.s.b as *const i32 as usize) };
    let pdeep = unsafe { (&o.u.s.u_deep.deep as *const i32 as usize) };
    let ptx = unsafe { (&o.tail.ts.tail_x as *const i32 as usize) };
    let pty = unsafe { (&o.tail.ts.tail_u.tail_y as *const i32 as usize) };

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
        process::exit(r);
    }

    r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        process::exit(r);
    }

    r = check_addressability();
    if r != 0 {
        process::exit(r);
    }

    process::exit(0);
}