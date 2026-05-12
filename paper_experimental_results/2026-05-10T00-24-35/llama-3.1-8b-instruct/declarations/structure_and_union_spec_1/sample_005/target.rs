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
    let v1 = V {
        union: union! {
            struct {
                i: 2,
                j: 0,
            },
            struct {
                k: 5,
                l: 0,
            },
        },
        m: 0,
    };

    assert_eq!(v1.union.struct_1.i, 2);
    assert_eq!(v1.union.struct_2.k, 5);

    assert_eq!(v1.union.struct_1.i, 2);
    assert_eq!(v1.union.struct_2.k, 5);

    assert!(true); // always returns 0
}