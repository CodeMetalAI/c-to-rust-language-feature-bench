#[derive(Default)]
struct Outer {
    base: i32,
    inner: Inner,
    tail: Tail,
}

#[derive(Default)]
struct Inner {
    a: i32,
    inner_union: InnerUnion,
    d: i32,
}

#[derive(Default)]
union InnerUnion {
    u_as_int: i32,
    inner_struct: InnerStruct,
    u_as_unsigned: u32,
}

#[derive(Default)]
struct InnerStruct {
    b: i32,
    deep_union: DeepUnion,
    c: i32,
}

#[derive(Default)]
union DeepUnion {
    deep: i32,
    deep_struct: DeepStruct,
}

#[derive(Default)]
struct DeepStruct {
    x: i32,
    y: i32,
}

#[derive(Default)]
union Tail {
    tail_i: i32,
    tail_struct: TailStruct,
}

#[derive(Default)]
struct TailStruct {
    tail_x: i32,
    tail_union: TailUnion,
}

#[derive(Default)]
union TailUnion {
    tail_y: i32,
    tail_struct: TailStruct2,
}

#[derive(Default)]
struct TailStruct2 {
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let mut o = Outer::default();
    o.base = 10;
    o.inner.a = 1;
    o.inner.inner_union.inner_struct.b = 2;
    o.inner.inner_union.inner_struct.deep_union.deep = 99;
    o.inner.inner_union.inner_struct.c = 3;
    o.inner.d = 4;
    o.tail.tail_struct.tail_x = 7;
    o.tail.tail_struct.tail_union.tail_y = 8;

    if o.base!= 10 {
        return 1;
    }
    if o.inner.a!= 1 {
        return 2;
    }
    if o.inner.inner_union.inner_struct.b!= 2 {
        return 3;
    }
    if o.inner.inner_union.inner_struct.deep_union.deep!= 99 {
        return 4;
    }
    if o.inner.inner_union.inner_struct.c!= 3 {
        return 5;
    }
    if o.inner.d!= 4 {
        return 6;
    }
    if o.tail.tail_struct.tail_x!= 7 {
        return 7;
    }
    if o.tail.tail_struct.tail_union.tail_y!= 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer::default();
    o.base = 0;

    o.inner.inner_union.inner_struct.deep_union.deep = 0x11223344;

    if o.inner.inner_union.inner_struct.deep_union.deep_struct.x!= 0x11223344 {
        return 20;
    }

    o.inner.inner_union.inner_struct.deep_union.deep_struct.x = 5;
    o.inner.inner_union.inner_struct.deep_union.deep_struct.y = 6;
    if o.inner.inner_union.inner_struct.deep_union.deep_struct.x!= 5 {
        return 21;
    }
    if o.inner.inner_union.inner_struct.deep_union.deep_struct.y!= 6 {
        return 22;
    }

    o.tail.tail_struct.tail_union.tail_struct.tail_p = 40;
    o.tail.tail_struct.tail_union.tail_struct.tail_q = 41;
    if o.tail.tail_struct.tail_union.tail_struct.tail_p!= 40 {
        return 23;
    }
    if o.tail.tail_struct.tail_union.tail_struct.tail_q!= 41 {
        return 24;
    }

    o.tail.tail_struct.tail_union.tail_y = -9;
    if o.tail.tail_struct.tail_union.tail_y!= -9 {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer::default();

    let pa = &o.inner.a as *const _ as usize;
    let pb = &o.inner.inner_union.inner_struct.b as *const _ as usize;
    let pdeep = &o.inner.inner_union.inner_struct.deep_union.deep as *const _ as usize;
    let ptx = &o.tail.tail_struct.tail_x as *const _ as usize;
    let pty = &o.tail.tail_struct.tail_union.tail_y as *const _ as usize;

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

    std::process::exit(0);
}