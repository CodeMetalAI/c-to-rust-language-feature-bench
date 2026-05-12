#[derive(Debug, Default)]
struct Inner {
    b: i32,
    deep: i32,
    c: i32,
}

#[derive(Debug, Default)]
struct InnerUnion {
    deep: i32,
    x: i32,
    y: i32,
}

#[derive(Debug, Default)]
struct InnerStruct {
    b: i32,
    inner_union: InnerUnion,
    c: i32,
}

#[derive(Debug, Default)]
struct InnerUnion2 {
    tail_y: i32,
    tail_p: i32,
    tail_q: i32,
}

#[derive(Debug, Default)]
struct InnerStruct2 {
    tail_x: i32,
    inner_union2: InnerUnion2,
}

#[derive(Debug, Default)]
struct Outer {
    base: i32,
    inner: InnerStruct,
    tail: InnerStruct2,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        inner: InnerStruct {
            b: 2,
            inner_union: InnerUnion { deep: 99,..Default::default() },
            c: 3,
           ..Default::default()
        },
        tail: InnerStruct2 {
            tail_x: 7,
            inner_union2: InnerUnion2 { tail_y: 8,..Default::default() },
           ..Default::default()
        },
       ..Default::default()
    };

    if o.base!= 10 {
        return 1;
    }
    if o.inner.b!= 2 {
        return 3;
    }
    if o.inner.inner_union.deep!= 99 {
        return 4;
    }
    if o.inner.c!= 3 {
        return 5;
    }
    if o.tail.tail_x!= 7 {
        return 7;
    }
    if o.tail.inner_union2.tail_y!= 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer::default();
    o.inner.inner_union.deep = 0x11223344;

    if o.inner.inner_union.x!= 0x11223344 {
        return 20;
    }

    o.inner.inner_union.x = 5;
    o.inner.inner_union.y = 6;
    if o.inner.inner_union.x!= 5 {
        return 21;
    }
    if o.inner.inner_union.y!= 6 {
        return 22;
    }

    o.tail.inner_union2.tail_p = 40;
    o.tail.inner_union2.tail_q = 41;
    if o.tail.inner_union2.tail_p!= 40 {
        return 23;
    }
    if o.tail.inner_union2.tail_q!= 41 {
        return 24;
    }

    o.tail.inner_union2.tail_y = -9;
    if o.tail.inner_union2.tail_y!= -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer::default();

    let pa = &o.inner.b as *const _ as usize;
    let pb = &o.inner.inner_union.deep as *const _ as usize;
    let pdeep = &o.inner.inner_union.deep as *const _ as usize;
    let ptx = &o.tail.tail_x as *const _ as usize;
    let pty = &o.tail.inner_union2.tail_y as *const _ as usize;

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
    if r!= 0 {
        std::process::exit(r);
    }

    let r = check_union_aliasing_via_flattened_names();
    if r!= 0 {
        std::process::exit(r);
    }

    let r = check_addressability();
    if r!= 0 {
        std::process::exit(r);
    }
}