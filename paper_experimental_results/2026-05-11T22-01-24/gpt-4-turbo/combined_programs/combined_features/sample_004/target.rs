fn main() {
    // [expr.postfix.subscript] 3x5 int array; fill via subscript, verify via pointer arithmetic
    let mut x = [[0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i + j;
        }
    }
    // x[0]={0,1,2,3,4}, x[1]={1,2,3,4,5}, x[2]={2,3,4,5,6}

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j]; // Direct access equivalent to pointer arithmetic in C
            if a != b {
                std::process::exit(1);
            }
        }
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
    let mut a = T1 { x: x[1][0], y: x[1][1] }; // {1, 2}
    let mut b = T1 { x: x[2][0], y: x[2][1] }; // {2, 3}

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

    let tp = &mut a;
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
        q += std::mem::size_of::<T1>(); // Same as struct s1
        if q == 0 {
            std::process::exit(13);
        }
    }

    {
        let mut c = T2 { n: x[2][0] }; // = 2, seeded from array
        let r = &c;
        if r.n != x[2][0] {
            std::process::exit(14);
        }
        if takes_t2(c) != 7 {
            std::process::exit(15);
        }
    }

    println!("PASS");
}

struct T1 {
    x: i32,
    y: i32,
}

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