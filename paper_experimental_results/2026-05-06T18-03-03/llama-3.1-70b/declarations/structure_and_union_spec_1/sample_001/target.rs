fn main() {
    #[repr(C)]
    struct V {
        #[repr(C)]
        union U {
            #[repr(C)]
            struct S1 {
                i: i32,
                j: i32,
            }
            #[repr(C)]
            struct S2 {
                k: i64,
                l: i64,
            }
        }
        m: i32,
    }

    let mut v1 = V {
        U: V::U { S1: V::U::S1 { i: 0, j: 0 } },
        m: 0,
    };

    unsafe {
        v1.U.S1.i = 2;
        v1.U.S2.k = 5;
    }

    if unsafe { v1.U.S1.i }!= 2 {
        std::process::exit(1);
    }

    if unsafe { v1.U.S2.k }!= 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}