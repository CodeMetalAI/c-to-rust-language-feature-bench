fn main() {
    // [expr.postfix.subscript] 3x5 int array; fill via subscript, verify via pointer arithmetic
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i as i32 + j as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = *unsafe { &*(x.as_ptr().offset(i as isize) as *const [i32; 5]).offset(j as isize) };
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    if (p1 as isize - p0 as isize) != 5 {
        std::process::exit(2);
    }

    // [expr.postfix.member] rvalue member access on returned struct, args from array
    if make_t1(x[1][0], x[1][1]).x != x[1][0] {
        std::process::exit(3);
    }
    if make_t1(x[1][0], x[1][1]).y != x[1][1] {
        std::process::exit(4);
    }
    if make_t1(x[0][3], x[1][2]).x + make_t1(x[2][0], x[0][4]).y != 7 {
        std::process::exit(5);
    }

    // [decl.type_defs] local struct variables seeded from array;
    let mut a = T1 { x: x[1][0], y: x[1][1] };
    let mut b = T1 { x: x[2][0], y: x[2][1] };
    let mut tp = &mut a;

    if takes_t1(a) != 4 {
        std::process::exit(6);
    }
    if takes_s1(a) != 5 {
        std::process::exit(7);
    }
    if takes_t1(b) != 6 {
        std::process::exit(8);
    }
    if takes_s1(b) != 7 {
        std::process::exit(9);
    }

    if takes_tp1(tp) != 101 {
        std::process::exit(10);
    }
    if a.x != 101 {
        std::process::exit(11);
    }
    if takes_int(a.x) != 105 {
        std::process::exit(12);
    }

    {
        let mut q = 0;
        q += std::mem::size_of::<T1>();
        q += std::mem::size_of::<T1>();
        if q == 0 {
            std::process::exit(13);
        }
    }

    {
        let mut c = T2 { n: x[2][0] };
        let r = &mut n;
        if r.n != x[2][0] {
            std::process::exit(14);
        }
        if takes_t2(n) != 7 {
            std::process::exit(15);
        }
    }

    println!("PASS");
}

#[derive(Copy, Clone)]
struct T1 {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct T2 {
    n: i32,
}

fn make_t1(x: i32, y: i32) -> T1 {
    T1 { x, y }
}

fn takes_t1(v: T1) -> i32 {
    v.x + v.y + 1
}

fn takes_s1(v: T1) -> i32 {
    v.x + v.y + 2
}

fn takes_tp1(p: &mut T1) -> i32 {
    p.x += 100;
    p.x
}

fn takes_int(v: i32) -> i32 {
    v + 4
}

fn takes_t2(v: T2) -> i32 {
    v.n + 5
}