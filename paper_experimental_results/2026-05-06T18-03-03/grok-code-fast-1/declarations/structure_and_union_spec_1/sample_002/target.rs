struct V {
    union_part: [u8; 16],
    m: i32,
}

fn main() -> i32 {
    let mut v1 = V {
        union_part: [0; 16],
        m: 0,
    };

    // v1.i = 2;
    let i_bytes = 2i32.to_le_bytes();
    v1.union_part[0..4].copy_from_slice(&i_bytes);

    // v1.w.k = 5;
    let k_bytes = 5i64.to_le_bytes();
    v1.union_part[0..8].copy_from_slice(&k_bytes);

    // if (v1.i != 2) return 1;
    let i = i32::from_le_bytes(v1.union_part[0..4].try_into().unwrap());
    if i != 2 {
        return 1;
    }

    // if (v1.w.k != 5) return 1;
    let k = i64::from_le_bytes(v1.union_part[0..8].try_into().unwrap());
    if k != 5 {
        return 1;
    }

    // return 0;
    0
}