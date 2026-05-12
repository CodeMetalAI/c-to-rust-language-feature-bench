struct V {
    union_data: UnionData,
    m: i32,
}

union UnionData {
    struct_data: StructData,
    w: W,
}

struct StructData {
    i: i32,
    j: i32,
}

struct W {
    k: i64,
    l: i64,
}

fn main() {
    let mut v1 = V {
        union_data: UnionData {
            struct_data: StructData { i: 2, j: 0 },
        },
        m: 0,
    };
    unsafe {
        v1.union_data.w.k = 5;
    }

    if unsafe { v1.union_data.struct_data.i } != 2 {
        std::process::exit(1);
    }

    if unsafe { v1.union_data.w.k } != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}