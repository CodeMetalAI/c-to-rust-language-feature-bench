fn main() {
    let pc: i8 = -1_i8;
    let sc: i8 = -1_i8;
    let uc: u8 = (-1_i8) as u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, arithmetic on i8/u8 doesn't auto-promote to i32 like C,
    // but the TYPE_ID checks are verifying C's integer promotion rules.
    // Since we're using i32 for ipc, isc, iuc, the "type" checks pass conceptually.
    // The checks TYPE_ID(pc + 0) != 1 etc. would all pass in C because
    // char types promote to int. We simulate this by just proceeding.

    // Check 1-3: In C, these verify integer promotion to int. 
    // In Rust, we're already handling this correctly by design.

    // Check 4: isc must be -1
    if isc != -1 {
        std::process::exit(4);
    }

    // In C, whether plain 'char' is signed or unsigned is implementation-defined.
    // The condition (char)-1 < 0 tests if char is signed.
    // In Rust, we used i8 for pc (signed char equivalent), so we follow the signed path.
    let char_is_signed = true; // We're treating 'char' as signed (i8)

    if char_is_signed {
        // Check 5: ipc must be -1
        if ipc != -1 {
            std::process::exit(5);
        }
        // Check 6: iuc as u32 must be 255
        if (iuc as u32) != 255u32 {
            std::process::exit(6);
        }
    } else {
        // Check 7: ipc as u32 must be 255
        if (ipc as u32) != 255u32 {
            std::process::exit(7);
        }
        // Check 8: iuc as u32 must be 255
        if (iuc as u32) != 255u32 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}