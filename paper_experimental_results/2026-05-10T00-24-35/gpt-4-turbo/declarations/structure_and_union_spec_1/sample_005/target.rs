fn main() {
    struct V {
        union_u: U,
        m: i32,
    }

    union U {
        unnamed: UnnamedStruct,
        w: NamedStruct,
    }
    
    #[derive(Copy, Clone)]
    struct UnnamedStruct {
        i: i32,
        j: i32,
    }
    
    #[derive(Copy, Clone)]
    struct NamedStruct {
        k: i64,
        l: i64,
    }

    let mut v1 = V {
        union_u: U {
            unnamed: UnnamedStruct { i: 0, j: 0 },
        },
        m: 0,
    };

    unsafe {
        v1.union_u.unnamed.i = 2;
        v1.union_u.w.k = 5;
    }

    unsafe {
        if v1.union_u.unnamed.i != 2 {
            std::process::exit(1);
        }

        if v1.union_u.w.k != 5 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}