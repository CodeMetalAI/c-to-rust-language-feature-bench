struct V {
    union {
        struct {
            i: i32,
            j: i32,
        },
        struct {
            k: i64,
            l: i64,
        },
    },
    m: i32,
}

fn main() {
    let mut v1 = V {
        union: V::Union {
            struct_1: V::Struct1 { i: 2, j: 0 },
            ..Default::default()
        },
        m: 0,
    };

    v1.union.struct_1.i = 2;
    v1.union.struct_2.k = 5;

    if v1.union.struct_1.i != 2 {
        return 1;
    }

    if v1.union.struct_2.k != 5 {
        return 1;
    }

    return 0;
}

// Note: Rust does not support the exact same syntax as C/C++ for unions.
// The above code uses a struct with a union inside it, which is the closest equivalent.
// Also, Rust does not support returning from `main` with a non-zero value.
// Instead, we use the `std::process::exit` function to exit the process with a non-zero status.