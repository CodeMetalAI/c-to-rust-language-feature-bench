fn main() {
    struct V {
        anonymous_union: AnonymousUnion,
        m: i32,
    }

    union AnonymousUnion {
        anonymous_struct_1: std::mem::ManuallyDrop<AnonymousStruct1>,
        w: std::mem::ManuallyDrop<W>,
    }

    struct AnonymousStruct1 {
        i: i32,
        j: i32,
    }

    struct W {
        k: i64,
        l: i64,
    }

    let mut v1 = V {
        anonymous_union: AnonymousUnion {
            anonymous_struct_1: std::mem::ManuallyDrop::new(AnonymousStruct1 { i: 0, j: 0 }),
        },
        m: 0,
    };

    unsafe {
        v1.anonymous_union.anonymous_struct_1.i = 2;
        v1.anonymous_union.w = std::mem::ManuallyDrop::new(W { k: 5, l: 0 });

        if v1.anonymous_union.anonymous_struct_1.i != 2 {
            std::process::exit(1);
        }

        if v1.anonymous_union.w.k != 5 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}