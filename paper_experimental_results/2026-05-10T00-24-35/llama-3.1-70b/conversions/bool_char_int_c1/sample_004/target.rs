enum E { E_NEG = -1, E_POS = 1 }

fn expect_type(got: u8, want: u8) -> bool { got == want }

fn main() {
    if!expect_type(std::mem::size_of_val(&(true as bool + 0)) as u8, 7) {
        std::process::exit(1);
    }
    if!expect_type(std::mem::size_of_val(&(1i8 as char + 0)) as u8, 7) {
        std::process::exit(2);
    }
    if!expect_type(std::mem::size_of_val(&(1i8 as i8 + 0)) as u8, 7) {
        std::process::exit(3);
    }
    if!expect_type(std::mem::size_of_val(&(1u8 as u8 + 0)) as u8, 7) {
        std::process::exit(4);
    }
    if!expect_type(std::mem::size_of_val(&(1i16 as i16 + 0)) as u8, 7) {
        std::process::exit(5);
    }
    if!expect_type(std::mem::size_of_val(&(1u16 as u16 + 0)) as u8, 7) {
        std::process::exit(6);
    }

    if!expect_type(std::mem::size_of_val(&(0i32 + 0u32)) as u8, 8) {
        std::process::exit(7);
    }
    if!expect_type(std::mem::size_of_val(&(0i64 + 0u64)) as u8, 10) {
        std::process::exit(8);
    }
    if!expect_type(std::mem::size_of_val(&(0i128 + 0u128)) as u8, 12) {
        std::process::exit(9);
    }

    if!expect_type(std::mem::size_of_val(&(0i32 + 0i64)) as u8, 9) {
        std::process::exit(10);
    }
    if!expect_type(std::mem::size_of_val(&(0i64 + 0i128)) as u8, 11) {
        std::process::exit(11);
    }
    if!expect_type(std::mem::size_of_val(&(0i32 + 0i128)) as u8, 11) {
        std::process::exit(12);
    }

    if!expect_type(std::mem::size_of_val(&(E::E_NEG as u32 + 0u32)) as u8, std::mem::size_of_val(&(0i32 + 0u32)) as u8) {
        std::process::exit(13);
    }
    if!expect_type(std::mem::size_of_val(&(E::E_NEG as i32 + 0i32)) as u8, std::mem::size_of_val(&(0i32 + 0i32)) as u8) {
        std::process::exit(14);
    }

    std::process::exit(0);
}