fn main() {
    // [expr.postfix.subscript] 3x5 int array; fill via subscript, verify via pointer arithmetic
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i + j;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = *x.get_unchecked(i).get_unchecked(j);
            if a!= b {
                std::process::exit(1);
            }
        }
    }

    let p0 = x.get_unchecked(0).get_unchecked(0);
    let p1 = x.get_unchecked(1).get_unchecked(0);
    if (p1 as *const i32).offset_from(p0 as *const i32) as isize!= 5 {
        std::process::exit(2);
    }

    // [expr.postfix.member] rvalue member access on returned struct, args from array
    if S1::new(x[1][0], x[1][1]).x!= x[1][0] {
        std::process::exit(3);
    }
    if S1::new(x[1][0], x[1][1]).y!= x[1][1] {
        std::process::exit(4);
    }
    if S1::new(x[0][3], x[1][2]).x + S1::new(x[2][0], x[0][4]).y!= 7 {
        std::process::exit(5);
    }

    // [decl.type_defs] local struct variables seeded from array;
    let mut a = S1::new(x[1][0], x[1][1]);
    let mut b = S1::new(x[2][0], x[2][1]);
    let mut tp = &mut a;

    if takes_s1(a)!= 4 {
        std::process::exit(6);
    }
    if takes_s1(a)!= 5 {
        std::process::exit(7);
    }
    if takes_s1(b)!= 6 {
        std::process::exit(8);
    }
    if takes_s1(b)!= 7 {
        std::process::exit(9);
    }

    if takes_tp1(tp)!= 101 {
        std::process::exit(10);
    }
    if a.x!= 101 {
        std::process::exit(11);
    }
    if takes_int(a.x)!= 105 {
        std::process::exit(12);
    }

    let mut q = 0;
    q += std::mem::size_of::<S1>();
    q += std::mem::size_of::<S1>();
    if q == 0 {
        std::process::exit(13);
    }

    let mut c = S2::new(x[2][0]);
    let r = &c;
    if r.n!= x[2][0] {
        std::process::exit(14);
    }
    if takes_s2(c)!= 7 {
        std::process::exit(15);
    }

    println!("PASS");
}

#[derive(Debug, Copy, Clone)]
struct S1 {
    x: i32,
    y: i32,
}

impl S1 {
    fn new(x: i32, y: i32) -> Self {
        S1 { x, y }
    }
}

fn takes_s1(v: S1) -> i32 {
    v.x + v.y + 1
}

fn takes_tp1(p: &mut S1) -> i32 {
    p.x += 100;
    p.x
}

fn takes_int(v: i32) -> i32 {
    v + 4
}

#[derive(Debug, Copy, Clone)]
struct S2 {
    n: i32,
}

impl S2 {
    fn new(n: i32) -> Self {
        S2 { n }
    }
}

fn takes_s2(v: S2) -> i32 {
    v.n + 5
}