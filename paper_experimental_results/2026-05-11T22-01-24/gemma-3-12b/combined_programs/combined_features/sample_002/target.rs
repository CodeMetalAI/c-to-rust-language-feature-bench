#[derive(Copy, Clone)]
struct T1 {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct T2 {
    n: i32,
}

struct S1 {
    x: i32,
    y: i32,
}

fn make_t1(x: i32, y: i32) -> T1 {
    T1 { x, y }
}

fn takes_t1(v: T1) -> i32 {
    v.x + v.y + 1
}

fn takes_s1(v: S1) -> i32 {
    v.x + v.y + 2
}

fn takes_tp1(p: *mut T1) -> i32 {
    unsafe {
        (*p).x += 100;
        (*p).x
    }
}

fn takes_int(v: i32) -> i32 {
    v + 4
}

fn takes_t2(v: T2) -> i32 {
    v.n + 5
}

fn main() {
    /* [expr.postfix.subscript] 3x5 int array; fill via subscript, verify via pointer arithmetic */
    let mut x = [[0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i + j;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = *(&x[i][j] as *const i32 as *const i32);
            if a != b {
                return 1;
            }
        }
    }

    let p0 = &x[0][0];
    let p1 = &x[1][0];
    if (p1 as *const i32).wrapping_sub(p0 as *const i32) / std::mem::size_of::<i32>() != 5 {
        return 2;
    }

    /* [expr.postfix.member] rvalue member access on returned struct, args from array */
    if make_t1(x[1][0], x[1][1]).x != x[1][0] {
        return 3;
    }
    if make_t1(x[1][0], x[1][1]).y != x[1][1] {
        return 4;
    }
    if make_t1(x[0][3], x[1][2]).x + make_t1(x[2][0], x[0][4]).y != 7 {
        return 5;
    }

    /* [decl.type_defs] local struct variables seeded from array;
       same variable passed to typedef-named and struct-tag-named params --
       requires Copy on T1/struct s1 in correct Rust translation */
    let mut a = T1 {
        x: x[1][0],
        y: x[1][1],
    };
    let mut b = S1 {
        x: x[2][0],
        y: x[2][1],
    };

    if takes_t1(a) != 4 {
        return 6;
    }
    if takes_s1(S1 { x: b.x, y: b.y }) != 5 {
        return 7;
    }

    if takes_t1(b.clone()) != 6 {
        return 8;
    }
    if takes_s1(S1 { x: b.x, y: b.y }) != 7 {
        return 9;
    }

    let tp = &mut a;
    if takes_tp1(tp as *mut T1) != 101 {
        return 10;
    }
    if a.x != 101 {
        return 11;
    }
    if takes_int(a.x) != 105 {
        return 12;
    }

    {
        let q = std::mem::size_of::<T1>() + std::mem::size_of::<S1>();
        if q == 0 {
            return 13;
        }
    }

    {
        let mut c = T2 { n: x[2][0] };
        let r = &mut c;
        if r.n != x[2][0] {
            return 14;
        }
        if takes_t2(c) != 7 {
            return 15;
        }
    }

    println!("PASS");
    0
}