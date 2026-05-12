struct S1 {
    x: i32,
}

struct S2 {
    x: i32,
}

fn takes_t1(v: S1) -> i32 {
    v.x + 1
}

fn takes_s1(v: S1) -> i32 {
    v.x + 2
}

fn takes_tp1(p: &mut S1) -> i32 {
    p.x += 3;
    p.x
}

fn takes_int(v: i32) -> i32 {
    v + 4
}

fn takes_t2(v: S2) -> i32 {
    v.x + 5
}

fn main() -> i32 {
    let mut a = S1 { x: 10 };
    let mut b = S1 { x: 20 };
    let p: &mut S1;

    if takes_t1(a) != 11 {
        return 1;
    }

    if takes_s1(a) != 12 {
        return 2;
    }

    if takes_t1(b) != 21 {
        return 3;
    }

    if takes_s1(b) != 22 {
        return 4;
    }

    p = &mut a;

    if takes_tp1(p) != 13 {
        return 5;
    }

    if a.x != 13 {
        return 6;
    }

    if takes_int(a.x) != 17 {
        return 7;
    }

    {
        let mut q = 0;
        q += std::mem::size_of::<S1>() as i32;
        q += std::mem::size_of::<S1>() as i32; // Same as S1 since they are identical
        if q == 0 {
            return 8;
        }
    }

    {
        let mut c = S2 { x: 30 };
        let r: &S2 = &c;
        if r.x != 30 {
            return 9;
        }
        if takes_t2(c) != 35 {
            return 10;
        }
    }

    0
}