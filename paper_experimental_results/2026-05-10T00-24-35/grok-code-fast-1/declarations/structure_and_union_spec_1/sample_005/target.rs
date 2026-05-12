#[repr(C)]
struct V {
    union_part: u64,
    m: i32,
}

fn main() {
    let mut v1 = V {
        union_part: 0,
        m: 0,
    };

    // v1.i = 2; set low 32 bits to 2
    v1.union_part = (v1.union_part & 0xFFFFFFFF00000000) | (2u64);

    // v1.w.k = 5; set whole u64 to 5
    v1.union_part = 5;

    // check v1.i
    let i = (v1.union_part & 0xFFFFFFFF) as i32;
    if i != 2 {
        std::process::exit(1);
    }

    // check v1.w.k
    let k = v1.union_part as i64;
    if k != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}