use std::mem;

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

fn main() {
    let mut x = [[0i32; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i as i32 + j as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x
                .iter()
                .flatten()
                .nth(i * 5 + j)
                .copied()
                .unwrap();
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0_index = 0i32;
    let p1_index = 5i32;
    if p1_index - p0_index != 5 {
        std::process::exit(2);
    }

    if make_t1(x[1][0], x[1][1]).x != x[1][0] {
        std::process::exit(3);
    }
    if make_t1(x[1][0], x[1][1]).y != x[1][1] {
        std::process::exit(4);
    }
    if make_t1(x[0][3], x[1][2]).x + make_t1(x[2][0], x[0][4]).y != 7 {
        std::process::exit(5);
    }

    let mut a = T1 { x: 0, y: 0 };
    let mut b = T1 { x: 0, y: 0 };
    a.x = x[1][0];
    a.y = x[1][1];
    b.x = x[2][0];
    b.y = x[2][1];

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

    {
        let tp = &mut a;
        if takes_tp1(tp) != 101 {
            std::process::exit(10);
        }
    }
    if a.x != 101 {
        std::process::exit(11);
    }
    if takes_int(a.x) != 105 {
        std::process::exit(12);
    }

    {
        let mut q = 0i32;
        q += mem::size_of::<T1>() as i32;
        q += mem::size_of::<T1>() as i32;
        if q == 0 {
            std::process::exit(13);
        }
    }

    {
        let mut c = T2 { n: 0 };
        c.n = x[2][0];
        if c.n != x[2][0] {
            std::process::exit(14);
        }
        if takes_t2(c) != 7 {
            std::process::exit(15);
        }
    }

    println!("PASS");
}