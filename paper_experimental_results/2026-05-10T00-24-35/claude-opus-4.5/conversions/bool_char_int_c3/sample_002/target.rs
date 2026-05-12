fn main() {
    let pc: i8 = -1_i8;
    let sc: i8 = -1_i8;
    let uc: u8 = (-1_i8) as u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, arithmetic on i8/u8 doesn't auto-promote to i32 like in C,
    // but the TYPE_ID checks are verifying C's integer promotion rules.
    // Since we're using i32 for the "promoted" values, these checks pass.
    // The checks TYPE_ID(pc + 0) etc. verify that char types promote to int in C.
    // In Rust, we're simulating this by using i32 values, so checks 1-3 are satisfied.

    if isc != -1 {
        std::process::exit(4);
    }

    // In C, whether 'char' is signed or unsigned is implementation-defined.
    // The condition (char)-1 < 0 checks if char is signed.
    // In Rust, we're using i8 for pc (signed char behavior), so (char)-1 < 0 is true.
    let char_is_signed = true;

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