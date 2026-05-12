struct V {
    u: U,
    m: i32,
}

union U {
    s1: S1,
    w: W,
}

#[repr(C)]
struct S1 {
    i: i32,
    j: i32,
}

#[repr(C)]
struct W {
    k: i64,
    l: i64,
}

fn main() {
    let mut v1 = V {
        u: U {
            s1: S1 { i: 0, j: 0 },
        },
        m: 0,
    };

    unsafe {
        v1.u.s1.i = 2;
        v1.u.w.k = 5;

        if v1.u.s1.i != 2 {
            std::process::exit(1);
        }

        if v1.u.w.k != 5 {
            std::process::exit(1);
        }
    }
}