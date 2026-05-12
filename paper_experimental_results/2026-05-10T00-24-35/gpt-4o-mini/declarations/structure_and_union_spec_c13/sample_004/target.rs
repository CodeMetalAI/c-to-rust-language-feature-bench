struct Outer {
    base: i32,
    a: i32,
    b: i32,
    deep: i32,
    c: i32,
    d: i32,
    tail_x: i32,
    tail_y: i32,
    tail_p: i32,
    tail_q: i32,
}

impl Outer {
    fn new() -> Self {
        Outer {
            base: 10,
            a: 1,
            b: 2,
            deep: 99,
            c: 3,
            d: 4,
            tail_x: 7,
            tail_y: 8,
            tail_p: 0,
            tail_q: 0,
        }
    }
}

fn check_designated_init() -> Result<(), i32> {
    let o = Outer::new();

    if o.base != 10 { return Err(1); }
    if o.a != 1 { return Err(2); }
    if o.b != 2 { return Err(3); }
    if o.deep != 99 { return Err(4); }
    if o.c != 3 { return Err(5); }
    if o.d != 4 { return Err(6); }
    if o.tail_x != 7 { return Err(7); }
    if o.tail_y != 8 { return Err(8); }

    Ok(())
}

fn check_union_aliasing_via_flattened_names() -> Result<(), i32> {
    let mut o = Outer {
        base: 0,
        a: 0,
        b: 0,
        deep: 0x11223344,
        c: 0,
        d: 0,
        tail_x: 0,
        tail_y: 0,
        tail_p: 0,
        tail_q: 0,
    };

    if o.deep != 0x11223344 { return Err(20); }

    o.tail_x = 5;
    o.tail_y = 6;
    if o.tail_x != 5 { return Err(21); }
    if o.tail_y != 6 { return Err(22); }

    o.tail_p = 40;
    o.tail_q = 41;
    if o.tail_p != 40 { return Err(23); }
    if o.tail_q != 41 { return Err(24); }

    o.tail_y = -9;
    if o.tail_y != -9 { return Err(25); }

    Ok(())
}

fn check_addressability() -> Result<(), i32> {
    let mut o = Outer {
        base: 0,
        a: 0,
        b: 0,
        deep: 0,
        c: 0,
        d: 0,
        tail_x: 0,
        tail_y: 0,
        tail_p: 0,
        tail_q: 0,
    };

    let pa = &o.a as *const _ as usize;
    let pb = &o.b as *const _ as usize;
    let pdeep = &o.deep as *const _ as usize;
    let ptx = &o.tail_x as *const _ as usize;
    let pty = &o.tail_y as *const _ as usize;

    if pa == 0 || pb == 0 || pdeep == 0 || ptx == 0 || pty == 0 {
        return Err(30);
    }
    if pa == pb {
        return Err(31);
    }
    if ptx == pty {
        return Err(32);
    }

    Ok(())
}

fn main() {
    let mut result = check_designated_init();
    if let Err(r) = result {
        std::process::exit(r);
    }

    result = check_union_aliasing_via_flattened_names();
    if let Err(r) = result {
        std::process::exit(r);
    }

    result = check_addressability();
    if let Err(r) = result {
        std::process::exit(r);
    }

    std::process::exit(0);
}