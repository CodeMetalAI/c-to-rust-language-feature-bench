// initialization_13.rs

fn main() {
    // Define a union-like struct using an enum to hold different types
    enum U {
        AnyMember(i32),
        UMember(u32),
        Bytes([u8; 4]),
    }

    // Initialize with AnyMember
    let x = U::AnyMember(42);

    // Match to check the value
    if let U::AnyMember(value) = x {
        if value != 42 {
            std::process::exit(1);
        }
    } else {
        std::process::exit(1);
    }

    // Scoped block
    {
        // Initialize with UMember
        let y = U::UMember(7);

        // Match to check the value
        if let U::UMember(value) = y {
            if value != 7 {
                std::process::exit(2);
            }
        } else {
            std::process::exit(2);
        }
    }
}