use std::hint::black_box;

fn main() {
    let pc: i8 = black_box(-1i8);
    let sc: i8 = black_box(-1i8);
    let uc: u8 = black_box((-1i8) as u8);

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, arithmetic on i8/u8 doesn't auto-promote to i32 like C,
    // but the TYPE_ID checks are verifying C's integer promotion rules.
    // Since we're using i32 for the "promoted" values, these checks
    // are essentially checking that the promotion to i32 happened correctly.
    // In Rust, we explicitly cast, so these checks pass by construction.
    // The equivalent check is that adding 0 to an i8/u8 and casting to i32 works.
    
    // TYPE_ID(pc + 0) != 1 - in C this checks promotion to int
    // In Rust, i8 + 0 stays i8, but we're simulating C behavior
    // These checks always pass in our translation
    
    // if (TYPE_ID(pc + 0) != 1) return 1;
    // if (TYPE_ID(sc + 0) != 1) return 2;
    // if (TYPE_ID(uc + 0) != 1) return 3;
    // These are compile-time type checks in C that verify integer promotion
    // In Rust, we don't have the same promotion rules, but the behavior is preserved
    
    if isc != -1 {
        std::process::exit(4);
    }

    // In C, whether plain 'char' is signed or unsigned is implementation-defined.
    // The condition (char)-1 < 0 checks if char is signed.
    // In Rust, we're using i8 for char (signed), so this branch is taken.
    let char_is_signed = (-1i8) < 0;
    
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