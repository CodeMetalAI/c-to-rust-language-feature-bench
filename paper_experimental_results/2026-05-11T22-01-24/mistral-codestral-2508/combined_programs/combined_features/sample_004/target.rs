#[derive(Copy, Clone)]
struct T1 {
    x: i32,
    y: i32,
}

type Tp1 = *mut T1;

#[derive(Copy, Clone)]
struct T2 {
    n: i32,
}

type Tp2 = *mut T2;

fn make_t1(x: i32, y: i32) -> T1 {
    T1 { x, y }
}

fn takes_t1(v: T1) -> i32 {
    v.x + v.y + 1
}

fn takes_s1(v: T1) -> i32 {
    v.x + v.y + 2
}

fn takes_tp1(p: Tp1) -> i32 {
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
    // [expr.postfix.subscript] 3x5 int array; fill via subscript, verify via pointer arithmetic
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i as i32 + j as i32;
        }
    }
    // x[0]={0,1,2,3,4}, x[1]={1,2,3,4,5}, x[2]={2,3,4,5,6}

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = unsafe { *(x.as_ptr().offset(i as isize * 5) as *const i32).offset(j as isize) };
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    if unsafe { p1.offset_from(p0) } != 5 {
        std::process::exit(2);
    }

    // [expr.postfix.member] rvalue member access on returned struct, args from array
    if make_t1(x[1][0], x[1][1]).x != x[1][0] {
        std::process::exit(3);
    } // .x == 1
    if make_t1(x[1][0], x[1][1]).y != x[1][1] {
        std::process::exit(4);
    } // .y == 2
    if make_t1(x[0][3], x[1][2]).x + make_t1(x[2][0], x[0][4]).y != 7 {
        std::process::exit(5);
    }
    // x[0][3]=3, x[1][2]=3, x[2][0]=2, x[0][4]=4: make_t1(3,3).x + make_t1(2,4).y = 3+4=7

    // [decl.type_defs] local struct variables seeded from array;
    // same variable passed to typedef-named and struct-tag-named params --
    // requires Copy on T1/struct s1 in correct Rust translation
    let mut a = T1 { x: 0, y: 0 };
    let mut b = T1 { x: 0, y: 0 };
    let mut tp: Tp1;
    a.x = x[1][0];
    a.y = x[1][1]; // {1, 2}
    b.x = x[2][0];
    b.y = x[2][1]; // {2, 3}

    if takes_t1(a) != 4 {
        std::process::exit(6);
    } // 1+2+1=4,  T1 param
    if takes_s1(a) != 5 {
        std::process::exit(7);
    } // 1+2+2=5,  struct s1 param, same var a
    if takes_t1(b) != 6 {
        std::process::exit(8);
    } // 2+3+1=6,  T1 param
    if takes_s1(b) != 7 {
        std::process::exit(9);
    } // 2+3+2=7,  struct s1 param, same var b

    tp = &mut a as *mut T1;
    if takes_tp1(tp) != 101 {
        std::process::exit(10);
    } // a.x: 1+100=101
    if a.x != 101 {
        std::process::exit(11);
    } // aliasing: original reflects mutation
    if takes_int(a.x) != 105 {
        std::process::exit(12);
    } // 101+4=105

    {
        let mut q = 0;
        q += std::mem::size_of::<T1>() as i32;
        q += std::mem::size_of::<T1>() as i32;
        if q == 0 {
            std::process::exit(13);
        }
    }

    {
        let mut c = T2 { n: 0 };
        let mut r: Tp2;
        c.n = x[2][0]; // = 2, seeded from array
        r = &mut c as *mut T2;
        if unsafe { (*r).n } != x[2][0] {
            std::process::exit(14);
        } // = 2
        if takes_t2(c) != 7 {
            std::process::exit(15);
        } // 2+5=7
    }

    println!("PASS");
    std::process::exit(0);
}