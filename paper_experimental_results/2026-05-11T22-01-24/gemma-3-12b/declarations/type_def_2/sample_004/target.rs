#[derive(Debug, Copy, Clone)]
struct s1 {
    x: i32,
}

type t1 = s1;
type tp1 = *mut s1;

#[derive(Debug, Copy, Clone)]
struct s2 {
    x: i32,
}

type t2 = s2;
type tp2 = *mut s2;

fn takes_t1(v: t1) -> i32 {
    v.x + 1
}

fn takes_s1(v: s1) -> i32 {
    v.x + 2
}

fn takes_tp1(p: *mut s1) -> i32 {
    unsafe {
        (*p).x += 3;
        (*p).x
    }
}

fn takes_int(v: i32) -> i32 {
    v + 4
}

fn takes_t2(v: t2) -> i32 {
    v.x + 5
}

fn main() {
    let mut a = s1 { x: 10 };
    let mut b = s1 { x: 20 };
    let mut p: *mut s1 = &mut a;

    if takes_t1(a) != 11 {
        return 1;
    }

    if takes_s1(b) != 22 {
        return 2;
    }

    if takes_t1(b) != 21 {
        return 3;
    }

    if takes_s1(b) != 22 {
        return 4;
    }

    unsafe {
        p = &mut a;
        if takes_tp1(p) != 13 {
            return 5;
        }
        if a.x != 13 {
            return 6;
        }
    }

    if takes_int(a.x) != 17 {
        return 7;
    }

    {
        let q = std::mem::size_of::<t1>() as i32 + std::mem::size_of::<s1>() as i32;
        if q == 0 {
            return 8;
        }
    }

    {
        let mut c = s2 { x: 30 };
        let mut r: *mut s2 = &mut c;
        if *r != 30 {
            return 9;
        }
        if takes_t2(c) != 35 {
            return 10;
        }
    }

    0
}