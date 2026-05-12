fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type((1 as i32 + 0) as i32, 7) {
        std::process::exit(1);
    }
    if !expect_type((1 as i8 as i32 + 0) as i32, 7) {
        std::process::exit(2);
    }
    if !expect_type((1 as i8 as i32 + 0) as i32, 7) {
        std::process::exit(3);
    }
    if !expect_type((1 as u8 as i32 + 0) as i32, 7) {
        std::process::exit(4);
    }
    if !expect_type((1 as i16 as i32 + 0) as i32, 7) {
        std::process::exit(5);
    }
    if !expect_type((1 as u16 as i32 + 0) as i32, 7) {
        std::process::exit(6);
    }

    if !expect_type((0 as i32 + 0 as u32) as i32, 8) {
        std::process::exit(7);
    }
    if !expect_type((0 as i64 as i32 + 0 as u64 as i32) as i32, 10) {
        std::process::exit(8);
    }
    if !expect_type((0 as i64 as i32 + 0 as u64 as i32) as i32, 12) {
        std::process::exit(9);
    }

    if !expect_type((0 as i32 + 0 as i64 as i32) as i32, 9) {
        std::process::exit(10);
    }
    if !expect_type((0 as i64 as i32 + 0 as i64 as i32) as i32, 11) {
        std::process::exit(11);
    }
    if !expect_type((0 as i32 + 0 as i64 as i32) as i32, 11) {
        std::process::exit(12);
    }

    if !expect_type((0 as i32 + 0 as u32) as i32, (0 as i32 + 0 as u32) as i32) {
        std::process::exit(13);
    }
    if !expect_type((0 as i32 + 0 as i32) as i32, (0 as i32 + 0 as i32) as i32) {
        std::process::exit(14);
    }
}