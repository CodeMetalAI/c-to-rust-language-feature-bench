struct InnerMost {
    x: i32,
    y: i32,
}

struct Inner {
    b: i32,
    deep: InnerMost,
    c: i32,
}

struct Middle {
    a: i32,
    inner: Option<Inner>,
    d: i32,
}

struct TailInnerMost {
    tail_p: i32,
    tail_q: i32,
}

struct Tail {
    tail_x: i32,
    tail_inner: Option<TailInnerMost>,
}

struct Outer {
    base: i32,
    middle: Middle,
    tail: Tail,
}

impl Outer {
    fn new() -> Outer {
        Outer {
            base: 0,
            middle: Middle {
                a: 0,
                inner: None,
                d: 0,
            },
            tail: Tail {
                tail_x: 0,
                tail_inner: None,
            },
        }
    }
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        middle: Middle {
            a: 1,
            inner: Some(Inner {
                b: 2,
                deep: InnerMost { x: 99, y: 99 },
                c: 3,
            }),
            d: 4,
        },
        tail: Tail {
            tail_x: 7,
            tail_inner: Some(TailInnerMost { tail_p: 8, tail_q: 8 }),
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.middle.a != 1 {
        return 2;
    }
    if let Some(inner) = &o.middle.inner {
        if inner.b != 2 {
            return 3;
        }
        if inner.deep.x != 99 {
            return 4;
        }
        if inner.c != 3 {
            return 5;
        }
    }
    if o.middle.d != 4 {
        return 6;
    }
    if o.tail.tail_x != 7 {
        return 7;
    }
    if let Some(tail_inner) = &o.tail.tail_inner {
        if tail_inner.tail_p != 8 {
            return 8;
        }
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer::new();
    o.middle.inner = Some(Inner {
        b: 0,
        deep: InnerMost {
            x: 0x11223344,
            y: 0,
        },
        c: 0,
    });

    if let Some(inner) = &o.middle.inner {
        if inner.deep.x != 0x11223344 {
            return 20;
        }
    }

    if let Some(inner) = o.middle.inner.as_mut() {
        inner.deep.x = 5;
        inner.deep.y = 6;
        if inner.deep.x != 5 {
            return 21;
        }
        if inner.deep.y != 6 {
            return 22;
        }
    }

    o.tail.tail_inner = Some(TailInnerMost { tail_p: 40, tail_q: 41 });
    if let Some(tail_inner) = &o.tail.tail_inner {
        if tail_inner.tail_p != 40 {
            return 23;
        }
        if tail_inner.tail_q != 41 {
            return 24;
        }
    }

    o.tail.tail_x = -9;
    if o.tail.tail_x != -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer::new();

    let pa = &o.middle.a as *const _ as usize;
    let pb = &o.middle.inner.as_ref().map(|i| i.b).unwrap_or(0) as *const _ as usize;
    let pdeep = &o.middle.inner.as_ref().map(|i| i.deep.x).unwrap_or(0) as *const _ as usize;
    let ptx = &o.tail.tail_x as *const _ as usize;
    let pty = &o.tail.tail_inner.as_ref().map(|i| i.tail_q).unwrap_or(0) as *const _ as usize;

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

fn main() -> i32 {
    let mut r;

    r = check_designated_init();
    if r != 0 {
        return r;
    }

    r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        return r;
    }

    r = check_addressability();
    if r != 0 {
        return r;
    }

    0
}