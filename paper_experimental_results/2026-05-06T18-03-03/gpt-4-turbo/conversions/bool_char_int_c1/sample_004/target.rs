fn main() {
    fn expect_type(got: usize, want: usize) -> bool {
        got == want
    }

    let type_id = |x: &dyn std::any::Any| -> usize {
        if x.is::<bool>() {
            1
        } else if x.is::<char>() {
            2
        } else if x.is::<i8>() {
            3
        } else if x.is::<u8>() {
            4
        } else if x.is::<i16>() {
            5
        } else if x.is::<u16>() {
            6
        } else if x.is::<i32>() {
            7
        } else if x.is::<u32>() {
            8
        } else if x.is::<i64>() {
            9
        } else if x.is::<u64>() {
            10
        } else if x.is::<i128>() {
            11
        } else if x.is::<u128>() {
            12
        } else {
            99
        }
    };

    if !expect_type(type_id(&(true as i32)), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id(&(1_i8 as i32)), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id(&(1_i8 as i32)), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id(&(1_u8 as i32)), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id(&(1_i16 as i32)), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id(&(1_u16 as i32)), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id(&(0_i32 + 0_u32)), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id(&(0_i64 + 0_u64)), 10) {
        std::process::exit(8);
    }
    if !expect_type(type_id(&(0_i128 + 0_u128)), 12) {
        std::process::exit(9);
    }

    if !expect_type(type_id(&(0_i32 + 0_i64)), 9) {
        std::process::exit(10);
    }
    if !expect_type(type_id(&(0_i64 + 0_i128)), 11) {
        std::process::exit(11);
    }
    if !expect_type(type_id(&(0_i32 + 0_i128)), 11) {
        std::process::exit(12);
    }

    if !expect_type(type_id(&(0_i32 + 0_u32)), type_id(&(0_i32 + 0_u32))) {
        std::process::exit(13);
    }
    if !expect_type(type_id(&(0_i32 + 0_i32)), type_id(&(0_i32 + 0_i32))) {
        std::process::exit(14);
    }
}