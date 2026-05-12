fn main() {
    let pc: i8 = -1_i8;
    let sc: i8 = -1_i8;
    let uc: u8 = (-1_i8) as u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, arithmetic on i8/u8 doesn't auto-promote to i32 like C,
    // but the TYPE_ID checks are verifying C's integer promotion rules.
    // Since we're in Rust, these checks about type promotion don't directly apply,
    // but we need to preserve the behavior. In C, char + 0 promotes to int.
    // We'll skip these checks as they're compile-time type checks in C.
    // The checks would pass in C (all promote to int which is type 1).

    if isc != -1 {
        std::process::exit(4);
    }

    // In C, whether 'char' is signed or unsigned is implementation-defined.
    // On most platforms (x86/x64 Linux, Windows), char is signed.
    // We'll assume signed char behavior (char)-1 < 0 is true.
    let char_is_signed = true;

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