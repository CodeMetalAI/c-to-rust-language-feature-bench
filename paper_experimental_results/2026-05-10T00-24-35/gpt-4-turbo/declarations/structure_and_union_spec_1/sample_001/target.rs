fn main() {
    #[derive(Default)]
    struct V {
        u: U,
        m: i32,
    }

    #[derive(Default)]
    union U {
        anon_struct: AnonStruct,
        w: W,
    }

    #[derive(Default)]
    struct AnonStruct {
        i: i32,
        j: i32,
    }

    #[derive(Default)]
    struct W {
        k: i64,
        l: i64,
    }

    let mut v1 = V::default();
    
    unsafe {
        v1.u.anon_struct.i = 2;
        v1.u.w.k = 5;
        
        if v1.u.anon_struct.i != 2 {
            std::process::exit(1);
        }

        if v1.u.w.k != 5 {
            std::process::exit(1);
        }
    }
}