use std::mem::size_of;
use std::process::exit;

#[derive(Copy, Clone)]
struct S1 {
    x: i32,
}
type T1 = S1;

#[derive(Copy, Clone)]
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

fn takes_tp1(p: &mut S1) -> i32 {
    p.x += 3;
    p.x
}

fn takes_int(v: i32) -> i32 {
    v + 4
}

fn takes_t2(v: T2) -> i32 {
    v.x + 5
}

fn main() {
    let mut a = S1 { x: 0 };
    let mut b = S1 { x: 0 };

    a.x = 10;
    b.x = 20;

    if takes_t1(a) != 11 {
        exit(1);
    }

    if takes_s1(a) != 12 {
        exit(2);
    }

    if takes_t1(b) != 21 {
        exit(3);
    }

    if takes_s1(b) != 22 {
        exit(4);
    }

    let res_tp1 = {
        let p = &mut a;
        takes_tp1(p)
    };
    if res_tp1 != 13 {
        exit(5);
    }

    if a.x != 13 {
        exit(6);
    }

    if takes_int(a.x) != 17 {
        exit(7);
    }

    {
        let mut q: i32 = 0;
        q += size_of::<T1>() as i32;
        q += size_of::<S1>() as i32;
        if q == 0 {
            exit(8);
        }
    }

    {
        let mut c = S2 { x: 30 };
        {
            let r = &mut c;
            if r.x != 30 {
                exit(9);
            }
        }
        if takes_t2(c) != 35 {
            exit(10);
        }
    }

    exit(0);
}