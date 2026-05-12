fn main() {
    #[repr(C)]
    struct V {
        #[repr(C)]
        union U {
            #[repr(C)]
            struct S {
                i: i32,
                j: i32,
            }
            #[repr(C)]
            struct W {
                k: i64,
                l: i64,
            }
        } u,
        m: i32,
    }

    let mut v1 = V { u: V::U { S: V::U::S { i: 0, j: 0 } }, m: 0 };

    unsafe {
        v1.u.S.i = 2;
        v1.u.W.k = 5;
    }

    if unsafe { v1.u.S.i }!= 2 {
        std::process::exit(1);
    }

    if unsafe { v1.u.W.k }!= 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}