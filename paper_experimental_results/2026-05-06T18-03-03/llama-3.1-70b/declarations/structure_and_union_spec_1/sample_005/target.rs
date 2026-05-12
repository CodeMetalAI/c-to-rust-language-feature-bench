fn main() {
    #[repr(C)]
    struct V {
        #[repr(C)]
        union U {
            struct {
                i: i32,
                j: i32,
            },
            w: struct {
                k: i64,
                l: i64,
            },
        },
        m: i32,
    }

    let mut v1 = V {
        U: unsafe { std::mem::zeroed() },
        m: 0,
    };

    unsafe {
        v1.U.i = 2;
        v1.U.w.k = 5;
    }

    if unsafe { v1.U.i }!= 2 {
        std::process::exit(1);
    }

    if unsafe { v1.U.w.k }!= 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}