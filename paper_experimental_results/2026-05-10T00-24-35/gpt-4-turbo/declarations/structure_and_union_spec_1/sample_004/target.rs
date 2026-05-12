fn main() {
    struct InnerStruct {
        i: i32,
        j: i32,
    }

    struct LongStruct {
        k: i64,
        l: i64,
    }

    union InnerUnion {
        short: InnerStruct,
        long: LongStruct,
    }

    struct OuterStruct {
        inner: InnerUnion,
        m: i32,
    }

    let mut outer = OuterStruct {
        inner: InnerUnion { short: InnerStruct { i: 2, j: 0 } },
        m: 0,
    };

    // Safe access to union by changing the active field
    unsafe {
        outer.inner.long.k = 5;
    }

    let i_value = unsafe { outer.inner.short.i };
    let k_value = unsafe { outer.inner.long.k };

    if i_value != 2 {
        std::process::exit(1);
    }

    if k_value != 5 {
        std::process::exit(1);
    }
}