// combined_features.rs
// Features threaded through a 3x5 int array whose values seed the struct operations:
//   expressions/array_subscripting  [expr.postfix.subscript]
//   expressions/struct_and_union_1  [expr.postfix.member]
//   declarations/type_def_2         [decl.type_defs]

struct S1 {
    x: i32,
    y: i32,
}

struct S2 {
    n: i32,
}

fn make_t1(x: i32, y: i32) -> S1 {
    S1 { x, y }
}

fn takes_t1(v: S1) -> i32 {
    v.x + v.y + 1
}

fn takes_s1(v: S1) -> i32 {
    v.x + v.y + 2
}

fn takes_tp1(p: &mut S1) -> i32 {
    p.x += 100;
    p.x
}

fn takes_int(v: i32) -> i32 {
    v + 4
}

fn takes_t2(v: S2) -> i32 {
    v.n + 5
}

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
            let b = *(&x[i][0] + j);
            if a!= b {
                println!("Error: a!= b");
                return;
            }
        }
    }

    let p0 = &x[0][0];
    let p1 = &x[1][0];
    if (p1 as *const _ as usize - p0 as *const _ as usize)!= 5 {
        println!("Error: p1 - p0!= 5");
        return;
    }

    // [expr.postfix.member] rvalue member access on returned struct, args from array
    if make_t1(x[1][0], x[1][1]).x!= x[1][0] {
        println!("Error: make_t1(x[1][0], x[1][1]).x!= x[1][0]");
        return;
    }
    if make_t1(x[1][0], x[1][1]).y!= x[1][1] {
        println!("Error: make_t1(x[1][0], x[1][1]).y!= x[1][1]");
        return;
    }
    if make_t1(x[0][3], x[1][2]).x + make_t1(x[2][0], x[0][4]).y!= 7 {
        println!("Error: make_t1(x[0][3], x[1][2]).x + make_t1(x[2][0], x[0][4]).y!= 7");
        return;
    }

    // [decl.type_defs] local struct variables seeded from array;
    // same variable passed to typedef-named and struct-tag-named params --
    // requires Copy on T1/struct s1 in correct Rust translation
    let mut a = S1 { x: 0, y: 0 };
    let mut b = S1 { x: 0, y: 0 };
    let mut tp = &mut a;
    a.x = x[1][0];
    a.y = x[1][1];
    b.x = x[2][0];
    b.y = x[2][1];

    if takes_t1(a)!= 4 {
        println!("Error: takes_t1(a)!= 4");
        return;
    }
    if takes_s1(a)!= 5 {
        println!("Error: takes_s1(a)!= 5");
        return;
    }
    if takes_t1(b)!= 6 {
        println!("Error: takes_t1(b)!= 6");
        return;
    }
    if takes_s1(b)!= 7 {
        println!("Error: takes_s1(b)!= 7");
        return;
    }

    *tp = &a;
    if takes_tp1(tp)!= 101 {
        println!("Error: takes_tp1(tp)!= 101");
        return;
    }
    if a.x!= 101 {
        println!("Error: a.x!= 101");
        return;
    }
    if takes_int(a.x)!= 105 {
        println!("Error: takes_int(a.x)!= 105");
        return;
    }

    {
        let q = 0;
        q += std::mem::size_of::<S1>();
        q += std::mem::size_of::<S1>();
        if q == 0 {
            println!("Error: q == 0");
            return;
        }
    }

    {
        let mut c = S2 { n: 0 };
        let mut r = &mut c;
        c.n = x[2][0];
        *r = &mut c;
        if r.n!= x[2][0] {
            println!("Error: r.n!= x[2][0]");
            return;
        }
        if takes_t2(c)!= 7 {
            println!("Error: takes_t2(c)!= 7");
            return;
        }
    }

    println!("PASS");
}