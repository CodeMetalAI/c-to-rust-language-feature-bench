fn main() {
    #[repr(C)]
    union InnerUnion {
        inner_struct: InnerStruct,
        w: WStruct,
    }

    #[repr(C)]
    struct InnerStruct {
        i: i32,
        j: i32,
    }

    #[repr(C)]
    struct WStruct {
        k: i64,
        l: i64,
    }

    #[repr(C)]
    struct V {
        inner: InnerUnion,
        m: i32,
    }

    let mut v1 = V {
        inner: InnerUnion {
            inner_struct: InnerStruct { i: 0, j: 0 },
        },
        m: 0,
    };

    unsafe {
        v1.inner.inner_struct.i = 2;
        v1.inner.w.k = 5;
    }

    if unsafe { v1.inner.inner_struct.i } != 2 {
        std::process::exit(1);
    }

    if unsafe { v1.inner.w.k } != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}