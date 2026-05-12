#[derive(Debug)]
struct Outer {
    base: i32,
    a: struct {
        a: i32,
        inner: struct {
            b: i32,
            inner_union: struct {
                deep: i32,
                inner_struct: struct {
                    x: i32,
                    y: i32,
                },
            },
            c: i32,
        },
        d: i32,
    },
    tail: struct {
        tail_x: i32,
        tail_union: struct {
            tail_y: i32,
            tail_inner_struct: struct {
                tail_p: i32,
                tail_q: i32,
            },
        },
    },
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        a: {
            let a_struct = struct {
                a: 1,
                inner: struct {
                    b: 2,
                    inner_union: struct {
                        deep: 99,
                        inner_struct: struct {
                            x: 0, //dummy init for x and y
                            y: 0,
                        },
                    },
                    c: 3,
                },
            };
            a_struct
        },
        tail: {
            let tail_struct = struct {
                tail_x: 7,
                tail_union: struct {
                    tail_y: 8,
                    tail_inner_struct: struct {
                        tail_p: 0, //dummy init for tail_p and tail_q
                        tail_q: 0,
                    },
                },
            };
            tail_struct
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.a.a != 1 {
        return 2;
    }
    if o.a.inner.b != 2 {
        return 3;
    }
    if o.a.inner.inner_union.deep != 99 {
        return 4;
    }
    if o.a.inner.c != 3 {
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
        a: {
            let a_struct = struct {
                a: 0,
                inner: struct {
                    b: 0,
                    inner_union: struct {
                        deep: 0x11223344,
                        inner_struct: struct { x: 0, y: 0 },
                    },
                    c: 0,
                },
            };
            a_struct
        },
        tail: {
            let tail_struct = struct {
                tail_x: 0,
                tail_union: struct {
                    tail_y: 0,
                    tail_inner_struct: struct { tail_p: 0, tail_q: 0 },
                },
            };
            tail_struct
        },
    };

    if o.a.inner.inner_union.inner_struct.x != 0x11223344 {
        return 20;
    }

    o.a.inner.inner_union.inner_struct.x = 5;
    o.a.inner.inner_union.inner_struct.y = 6;
    if o.a.inner.inner_union.inner_struct.x != 5 {
        return 21;
    }
    if o.a.inner.inner_union.inner_struct.y != 6 {
        return 22;
    }

    o.tail.tail_union.tail_inner_struct.tail_p = 40;
    o.tail.tail_union.tail_inner_struct.tail_q = 41;
    if o.tail.tail_union.tail_inner_struct.tail_p != 40 {
        return 23;
    }
    if o.tail.tail_union.tail_inner_struct.tail_q != 41 {
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
        a: {
            let a_struct = struct {
                a: 0,
                inner: struct {
                    b: 0,
                    inner_union: struct {
                        deep: 0,
                        inner_struct: struct { x: 0, y: 0 },
                    },
                    c: 0,
                },
            };
            a_struct
        },
        tail: {
            let tail_struct = struct {
                tail_x: 0,
                tail_union: struct {
                    tail_y: 0,
                    tail_inner_struct: struct { tail_p: 0, tail_q: 0 },
                },
            };
            tail_struct
        },
    };

    let pa: *const i32 = &o.a.a;
    let pb: *const i32 = &o.a.inner.b;
    let pdeep: *const i32 = &o.a.inner.inner_union.deep;
    let ptx: *const i32 = &o.tail.tail_x;
    let pty: *const i32 = &o.tail.tail_union.tail_y;

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