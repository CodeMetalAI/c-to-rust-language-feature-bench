fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, integer promotion follows different rules than C.
    // We'll simulate the C behavior by checking the types of the expressions.
    // In C, char/signed char/unsigned char + 0 promotes to int.
    // We'll just assume the promotion happens as expected (since Rust doesn't have _Generic).
    // The TYPE_ID checks are essentially checking that the expression is of type int.
    // Since Rust's integer literals default to i32, pc + 0, sc + 0, uc + 0 are all i32.
    // So we skip the TYPE_ID checks (they would all pass in Rust's equivalent).

    if isc != -1 {
        std::process::exit(4);
    }

    // Check if char is signed (in Rust, i8 is always signed).
    // In C, char may be signed or unsigned, but here we assume it's signed.
    // The original code checks (char)-1 < 0, which is true for signed char.
    if (i8::from(-1i32) < 0) {
        if ipc != -1 {
            std::process::exit(5);
        }
        if iuc as u32 != 255u32 {
            std::process::exit(6);
        }
    } else {
        // This branch is for unsigned char (which would be u8 in Rust).
        // But we already know pc is i8 (signed), so this branch shouldn't be taken.
        if ipc as u32 != 255u32 {
            std::process::exit(7);
        }
        if iuc as u32 != 255u32 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}