fn main() {
    // Rust has no direct union initialization syntax like C's designated initializers.
    // Instead, we use explicit field initialization:
    let x = U { any_member: 42 };

    if x.any_member != 42 {
        std::process::exit(1);
    }

    // A new scope for the second union instance
    {
        let y = U { u_member: 7 };
        if y.u_member != 7 {
            std::process::exit(2);
        }
    }

    // If all checks pass, return 0 (success)
    std::process::exit(0);
}

// Defining a union similar to the C union
union U {
    any_member: i32,
    u_member: u32,
    bytes: [u8; 4],
}