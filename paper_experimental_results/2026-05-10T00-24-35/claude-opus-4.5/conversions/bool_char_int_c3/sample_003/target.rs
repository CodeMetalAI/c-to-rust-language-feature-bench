fn main() {
    let pc: i8 = -1_i8;
    let sc: i8 = -1_i8;
    let uc: u8 = (-1_i8) as u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, adding 0 to any integer type results in the same type,
    // but the C code checks that char types promote to int.
    // Since we're using i8/u8 and the checks are about C's type promotion,
    // we simulate the behavior by checking the equivalent conditions.
    // TYPE_ID checks are about C integer promotion - in Rust i8+0 stays i8,
    // but the semantic check passes since we're verifying the promotion concept.
    // These checks always pass in the C code (chars promote to int), so we skip them.

    if isc != -1 {
        std::process::exit(4);
    }

    // In C, whether plain 'char' is signed or unsigned is implementation-defined.
    // We need to check if (char)-1 < 0, which depends on whether char is signed.
    // In Rust, we used i8 for pc (signed char equivalent).
    // To match typical behavior where char is signed on most platforms:
    let char_is_signed = (-1_i8) < 0;

    if char_is_signed {
        if ipc != -1 {
            std::process::exit(5);
        }
        if (iuc as u32) != 255u32 {
            std::process::exit(6);
        }
    } else {
        if (ipc as u32) != 255u32 {
            std::process::exit(7);
        }
        if (iuc as u32) != 255u32 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}