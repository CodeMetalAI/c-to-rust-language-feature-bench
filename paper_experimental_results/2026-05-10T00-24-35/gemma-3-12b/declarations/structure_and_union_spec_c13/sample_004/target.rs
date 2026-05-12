#[repr(C)]
struct Outer {
    base: i32,
    a: Nested1,
    tail: Tail,
}

#[repr(C)]
struct Nested1 {
    a: i32,
    b: Nested2,
    d: i32,
}

#[repr(C)]
struct Nested2 {
    b: i32,
    deep: Union1,
    c: i32,
}

#[repr(C)]
union Union1 {
    u_as_int: i32,
    u_as_unsigned: u32,
}

#[repr(C)]
struct Deep {
    x: i32,
    y: i32,
}

#[repr(C)]
struct Tail {
    tail_x: i32,
    tail_y: Union2,
}

#[repr(C)]
union Union2 {
    tail_y: i32,
    tail_nested: Nested3,
}

#[repr(C)]
struct Nested3 {
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        a: Nested1 {
            a: 1,
            b: Nested2 {
                b: 2,
                deep: Union1 {
                    u_as_int: 99,
                },
                c: 3,
            },
            d: 4,
        },
        tail: Tail {
            tail_x: 7,
            tail_y: Union2 {
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
    if o.a.b.b != 2 {
        return 3;
    }
    if o.a.b.deep.u_as_int != 99 {
        return 4;
    }
    if o.a.b.c != 3 {
        return 5;
    }
    if o.a.d != 4 {
        return 6;
    }
    if o.tail.tail_x != 7 {
        return 7;
    }
    if o.tail.tail_y.tail_y != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        a: Nested1 {
            a: 0,
            b: Nested2 {
                b: 0,
                deep: Union1 {
                    u_as_int: 0,
                },
                c: 0,
            },
            d: 0,
        },
        tail: Tail {
            tail_x: 0,
            tail_y: Union2 {
                tail_y: 0,
            },
        },
    };

    o.a.b.deep.u_as_int = 0x11223344 as i32;

    if o.a.b.deep.u_as_int != 0x11223344 as i32 {
        return 20;
    }

    o.a.b.deep.u_as_int = 5;
    o.tail.tail_x = 6;
    if o.a.b.deep.u_as_int != 5 {
        return 21;
    }
    if o.tail.tail_x != 6 {
        return 22;
    }

    o.tail.tail_y.tail_nested.tail_p = 40;
    o.tail.tail_y.tail_nested.tail_q = 41;
    if o.tail.tail_y.tail_nested.tail_p != 40 {
        return 23;
    }
    if o.tail.tail_y.tail_nested.tail_q != 41 {
        return 24;
    }

    o.tail.tail_y.tail_y = -9;
    if o.tail.tail_y.tail_y != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer {
        base: 0,
        a: Nested1 {
            a: 0,
            b: Nested2 {
                b: 0,
                deep: Union1 {
                    u_as_int: 0,
                },
                c: 0,
            },
            d: 0,
        },
        tail: Tail {
            tail_x: 0,
            tail_y: Union2 {
                tail_y: 0,
            },
        },
    };

    let pa: *const i32 = &o.a.a as *const i32;
    let pb: *const i32 = &o.a.b.b as *const i32;
    let pdeep: *const i32 = &o.a.b.deep.u_as_int as *const i32;
    let ptx: *const i32 = &o.tail.tail_x as *const i32;
    let pty: *const i32 = &o.tail.tail_y.tail_y as *const i32;

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