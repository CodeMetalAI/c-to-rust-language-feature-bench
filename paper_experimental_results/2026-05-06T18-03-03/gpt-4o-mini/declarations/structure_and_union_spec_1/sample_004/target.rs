struct V {
    union_data: UnionData,
    m: i32,
}

union UnionData {
    ij: IJ,
    w: W,
}

struct IJ {
    i: i32,
    j: i32,
}

struct W {
    k: i64,
    l: i64,
}

fn main() -> i32 {
    let mut v1 = V {
        union_data: UnionData { ij: IJ { i: 0, j: 0 } },
        m: 0,
    };

    unsafe {
        v1.union_data.ij.i = 2;
        v1.union_data.w.k = 5;
    }

    if unsafe { v1.union_data.ij.i } != 2 {
        return 1;
    }

    if unsafe { v1.union_data.w.k } != 5 {
        return 1;
    }

    return 0;
}