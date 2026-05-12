#[derive(Clone, Copy)]
struct S1 {
    x: i32,
}

type T1 = S1;

#[derive(Clone, Copy)]
struct S2 {
    x: i32,
}

type T2 = S2;

fn takes_t1(v: T1) -> i32 {
    v.x + 1
}

fn takes_s1(v: S1) -> i32 {
    v.x + 2
}

fn takes_tp1(p: &mut T1) -> i32 {
    p.x += 3;
    p.x
}

fn takes_int(v: i32) -> i32 {
    v + 4
}

fn takes_t2(v: T2) -> i32 {
    v.x + 5
}

fn main() -> i32 {
    let mut a: T1 = S1 { x: 10 };
    let b: S1 = S1 { x: 20 };

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

    if takes_tp1(&mut a) != 13 {
        return 5;
    }

    if a.x != 13 {
        return 6;
    }

    if takes_int(a.x) != 17 {
        return 7;
    }

    {
        let mut q: i32 = 0;
        q += std::mem::size_of::<T1>() as i32;
        q += std::mem::size_of::<S1>() as i32;
        if q == 0 {
            return 8;
        }
    }

    {
        let c: T2 = S2 { x: 30 };
        let r: &T2 = &c;
        if r.x != 30 {
            return 9;
        }
        if takes_t2(c) != 35 {
            return 10;
        }
    }

    0
}