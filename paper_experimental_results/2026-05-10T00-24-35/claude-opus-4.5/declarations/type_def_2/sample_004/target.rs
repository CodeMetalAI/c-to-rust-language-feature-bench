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

fn main() {
    let mut a = S1 { x: 0 };
    let b = S1 { x: 0 };

    a.x = 10;
    let b = S1 { x: 20 };

    if takes_t1(S1 { x: a.x }) != 11 {
        std::process::exit(1);
    }

    if takes_s1(S1 { x: a.x }) != 12 {
        std::process::exit(2);
    }

    if takes_t1(S1 { x: b.x }) != 21 {
        std::process::exit(3);
    }

    if takes_s1(S1 { x: b.x }) != 22 {
        std::process::exit(4);
    }

    if takes_tp1(&mut a) != 13 {
        std::process::exit(5);
    }

    if a.x != 13 {
        std::process::exit(6);
    }

    if takes_int(a.x) != 17 {
        std::process::exit(7);
    }

    {
        let mut q: i32 = 0;
        q += std::mem::size_of::<S1>() as i32;
        q += std::mem::size_of::<S1>() as i32;
        if q == 0 {
            std::process::exit(8);
        }
    }

    {
        let c = S2 { x: 30 };
        let r = &c;
        if r.x != 30 {
            std::process::exit(9);
        }
        if takes_t2(S2 { x: c.x }) != 35 {
            std::process::exit(10);
        }
    }

    std::process::exit(0);
}