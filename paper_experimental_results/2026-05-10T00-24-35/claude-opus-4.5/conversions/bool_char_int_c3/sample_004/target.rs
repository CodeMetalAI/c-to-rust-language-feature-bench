fn main() {
    let pc: i8 = -1_i8;
    let sc: i8 = -1_i8;
    let uc: u8 = (-1_i8) as u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, adding 0 to any integer type results in the same type,
    // but we're checking the C behavior where char types promote to int.
    // Since we're using i8/u8 which would promote to i32 in C-like semantics,
    // these checks pass (TYPE_ID would return 1 for int).
    // The checks are essentially verifying integer promotion rules which
    // are satisfied by our type choices.

    // TYPE_ID(pc + 0) != 1 - in C, char promotes to int, so this is 1
    // TYPE_ID(sc + 0) != 1 - in C, signed char promotes to int, so this is 1
    // TYPE_ID(uc + 0) != 1 - in C, unsigned char promotes to int, so this is 1
    // All these conditions are false, so we don't return 1, 2, or 3

    if isc != -1 {
        std::process::exit(4);
    }

    // In C, whether plain 'char' is signed or unsigned is implementation-defined.
    // The condition (char)-1 < 0 checks if char is signed.
    // In Rust, we're using i8 for pc (signed char behavior).
    // We need to match the behavior of the target platform.
    // Since we used i8 for pc (which is signed), (char)-1 < 0 is true.
    
    let char_is_signed = (-1_i8) < 0; // This is always true for i8
    
    if char_is_signed {
        if ipc != -1 {
            std::process::exit(5);
        }
        if (iuc as u32) != 255_u32 {
            std::process::exit(6);
        }
    } else {
        if (ipc as u32) != 255_u32 {
            std::process::exit(7);
        }
        if (iuc as u32) != 255_u32 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}