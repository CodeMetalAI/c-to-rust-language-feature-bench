struct S1 {
    x: i32,
}

struct S2 {
    x: i32,
}

fn takes_s1(v: S1) -> i32 {
    v.x + 2
}

fn takes_t1(v: S1) -> i32 {
    v.x + 1
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
    let mut a = S1 { x: 10 };
    let b = S1 { x: 20 };

    if takes_t1(a) != 11 {
        std::process::exit(1);
    }

    if takes_s1(a) != 12 {
        std::process::exit(2);
    }

    if takes_t1(b) != 21 {
        std::process::exit(3);
    }

    if takes_s1(b) != 22 {
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

    let q = std::mem::size_of::<S1>() + std::mem::size_of::<S1>();
    if q == 0 {
        std::process::exit(8);
    }

    let mut c = S2 { x: 30 };
    let r = &mut c;
    if r.x != 30 {
        std::process::exit(9);
    }

    if takes_t2(c) != 35 {
        std::process::exit(10);
    }
}