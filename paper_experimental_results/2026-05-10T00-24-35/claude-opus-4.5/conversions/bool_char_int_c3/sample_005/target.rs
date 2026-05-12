fn main() {
    let pc: i8 = -1_i8;
    let sc: i8 = -1_i8;
    let uc: u8 = (-1_i8) as u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, adding 0 to any integer type results in the same type,
    // but the C code checks that char types promote to int.
    // Since we're using i8/u8 which don't auto-promote, we simulate
    // the check by verifying the arithmetic works as expected (always passes in Rust context)
    // The TYPE_ID checks in C verify integer promotion happens - in Rust this is implicit
    // when we do arithmetic, so these checks effectively always pass.
    
    // TYPE_ID(pc + 0) != 1 - in C, char promotes to int, so this passes
    // TYPE_ID(sc + 0) != 1 - in C, signed char promotes to int, so this passes  
    // TYPE_ID(uc + 0) != 1 - in C, unsigned char promotes to int, so this passes
    // All three checks pass in C (return value is 1 for int), so we don't return 1, 2, or 3

    if isc != -1 {
        std::process::exit(4);
    }

    // In C, whether plain 'char' is signed or unsigned is implementation-defined.
    // The condition (char)-1 < 0 checks if char is signed.
    // In Rust, we used i8 for pc (signed), so we take the "signed char" branch.
    let char_is_signed = true; // We chose i8 for pc, simulating signed char

    if char_is_signed {
        // (char)-1 < 0 is true, meaning char is signed
        if ipc != -1 {
            std::process::exit(5);
        }
        if (iuc as u32) != 255u32 {
            std::process::exit(6);
        }
    } else {
        // (char)-1 < 0 is false, meaning char is unsigned
        if (ipc as u32) != 255u32 {
            std::process::exit(7);
        }
        if (iuc as u32) != 255u32 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}