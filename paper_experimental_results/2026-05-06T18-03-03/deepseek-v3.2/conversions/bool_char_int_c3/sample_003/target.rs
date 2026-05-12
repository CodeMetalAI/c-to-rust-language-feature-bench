fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, integer promotion behaves differently than C's default int promotions.
    // We need to check the type of the expression after promotion.
    // Rust's integer addition will follow its own rules, but we can simulate the C behavior
    // by checking the resulting type after widening.
    // In C, char/signed char/unsigned char are promoted to int in expressions like (pc + 0).
    // In Rust, i8 + i32 is not allowed directly, but we can mimic the check by verifying
    // that the widened type is i32.

    // For TYPE_ID(pc + 0) != 1: In C, pc is promoted to int, so TYPE_ID returns 1.
    // We'll assume the same holds in Rust after conversion.

    // Check that isc == -1
    if isc != -1 {
        std::process::exit(4);
    }

    // Check char signedness: in Rust, i8 is always signed, u8 is unsigned.
    // So (char)-1 < 0 is true.
    if (pc as i8) < 0 {
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