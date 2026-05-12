fn expect_type(got: usize, want: usize) -> bool {
    got == want
}

fn main() {
    if !expect_type((1u8 + 0) as usize, 7) {
        std::process::exit(1);
    }
    if !expect_type((1i8 + 0) as usize, 7) {
        std::process::exit(2);
    }
    if !expect_type((1u8 + 0) as usize, 7) {
        std::process::exit(3);
    }
    if !expect_type((1u8 + 0) as usize, 7) {
        std::process::exit(4);
    }
    if !expect_type((1i16 + 0) as usize, 7) {
        std::process::exit(5);
    }
    if !expect_type((1u16 + 0) as usize, 7) {
        std::process::exit(6);
    }

    if !expect_type((0i32 + 0u32) as usize, 8) {
        std::process::exit(7);
    }
    if !expect_type((0i64 + 0u64) as usize, 10) {
        std::process::exit(8);
    }
    if !expect_type((0i64 + 0u64) as usize, 12) {
        std::process::exit(9);
    }

    if !expect_type((0i32 + 0i64) as usize, 9) {
        std::process::exit(10);
    }
    if !expect_type((0i64 + 0i128) as usize, 11) {
        std::process::exit(11);
    }
    if !expect_type((0i32 + 0i128) as usize, 11) {
        std::process::exit(12);
    }

    if !expect_type((0i32 + 0u32) as usize, (0i32 + 0u32) as usize) {
        std::process::exit(13);
    }
    if !expect_type((0i32 + 0) as usize, (0i32 + 0) as usize) {
        std::process::exit(14);
    }

    std::process::exit(0);
}