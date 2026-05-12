#[derive(Debug)]
struct v {
    union_field: Union,
    m: i32,
}

#[derive(Debug)]
union Union {
    struct_1 {
        i: i32,
        j: i32,
    },
    struct_2 {
        k: i64,
        l: i64,
    },
    w: struct_2,
}

fn main() {
    let mut v1 = v {
        union_field: Union {
            struct_1: Union::struct_1 { i: 0, j: 0 },
        },
        m: 0,
    };

    v1.union_field.struct_1.i = 2;
    v1.union_field.struct_2.k = 5;

    if v1.union_field.struct_1.i != 2 {
        return 1;
    }

    if v1.union_field.struct_2.k != 5 {
        return 1;
    }

    0
}