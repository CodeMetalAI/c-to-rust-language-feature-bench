#[repr(C)]
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

static CHECK_DESIGNATED_INIT: fn() -> i32 = || {
    let o = Outer {
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
    };

    if o.base != 10 {
        return 1;
    }
    if o.a != 1 {
        return 2;
    }
    if o.b != 2 {
        return 3;
    }
    if o.deep != 99 {
        return 4;
    }
    if o.c != 3 {
        return 5;
    }
    if o.d != 4 {
        return 6;
    }
    if o.tail_x != 7 {
        return 7;
    }
    if o.tail_y != 8 {
        return 8;
    }

    0
};

static CHECK_UNION_ALIASING_VIA_FLATTENED_NAMES: fn() -> i32 = || {
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
    o.deep = 0x11223344 as i32;

    if o.deep != 0x11223344 {
        return 20;
    }

    o.deep = 5;
    o.tail_x = 6;
    if o.deep != 5 {
        return 21;
    }
    if o.tail_x != 6 {
        return 22;
    }

    o.tail_p = 40;
    o.tail_q = 41;
    if o.tail_p != 40 {
        return 23;
    }
    if o.tail_q != 41 {
        return 24;
    }

    o.tail_y = -9;
    if o.tail_y != -9 {
        return 25;
    }

    0
};

static CHECK_ADDRESSABILITY: fn() -> i32 = || {
    let o = Outer {
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

    let pa = &o.a as *const i32 as uptr;
    let pb = &o.b as *const i32 as uptr;
    let pdeep = &o.deep as *const i32 as uptr;
    let ptx = &o.tail_x as *const i32 as uptr;
    let pty = &o.tail_y as *const i32 as uptr;

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
};

type uptr = u64;

fn main() -> i32 {
    let mut r = CHECK_DESIGNATED_INIT();
    if r != 0 {
        return r;
    }

    r = CHECK_UNION_ALIASING_VIA_FLATTENED_NAMES();
    if r != 0 {
        return r;
    }

    r = CHECK_ADDRESSABILITY();
    if r != 0 {
        return r;
    }

    0
}