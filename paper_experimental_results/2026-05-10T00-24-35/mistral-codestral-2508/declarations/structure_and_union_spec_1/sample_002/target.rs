fn main() {
    #[repr(C)]
    struct V {
        union_data: UnionData,
        m: i32,
    }

    #[repr(C)]
    union UnionData {
        i: i32,
        w: W,
    }

    #[repr(C)]
    struct W {
        k: i64,
        l: i64,
    }

    let mut v1 = V {
        union_data: UnionData { i: 0, w: W { k: 0, l: 0 } },
        m: 0,
    };

    unsafe {
        v1.union_data.i = 2;
        v1.union_data.w.k = 5;
    }

    if unsafe { v1.union_data.i } != 2 {
        std::process::exit(1);
    }

    if unsafe { v1.union_data.w.k } != 5 {
        std::process::exit(1);
    }
}