#[derive(Clone, Copy)]
struct S1 {
    x: i32,
}

#[derive(Clone, Copy)]
struct S2 {
    x: i32,
}

type T1 = S1;
type Tp1 = &mut S1;
type T2 = S2;
type Tp2 = &S2;

fn takes_t1(v: T1) -> i32 {
    v.x + 1
}

fn takes_s1(v: S1) -> i32 {
    v.x + 2
}

fn takes_tp1(p: Tp1) -> i32 {
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
    let mut exit_code = 0;

    let mut a: T1 = S1 { x: 10 };
    let b: S1 = S1 { x: 20 };

    if takes_t1(a) != 11 {
        exit_code = 1;
    } else if takes_s1(a) != 12 {
        exit_code = 2;
    } else if takes_t1(b) != 21 {
        exit_code = 3;
    } else if takes_s1(b) != 22 {
        exit_code = 4;
    } else {
        let p: Tp1 = &mut a;
        if takes_tp1(p) != 13 {
            exit_code = 5;
        } else if a.x != 13 {
            exit_code = 6;
        } else if takes_int(a.x) != 17 {
            exit_code = 7;
        } else {
            {
                let mut q: i32 = 0;
                q += std::mem::size_of::<T1>() as i32;
                q += std::mem::size_of::<S1>() as i32;
                if q == 0 {
                    exit_code = 8;
                }
            }
            if exit_code == 0 {
                {
                    let c: T2 = S2 { x: 30 };
                    let r: Tp2 = &c;
                    if r.x != 30 {
                        exit_code = 9;
                    } else if takes_t2(c) != 35 {
                        exit_code = 10;
                    }
                }
            }
        }
    }

    std::process::exit(exit_code);
}