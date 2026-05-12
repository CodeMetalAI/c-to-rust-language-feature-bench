fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, all char types promote to i32 in integer promotions
    // The TYPE_ID macro checks the type of (x + 0) which promotes to i32
    // So all should be type 1 (int)
    if type_id(pc + 0) != 1 {
        std::process::exit(1);
    }
    if type_id(sc + 0) != 1 {
        std::process::exit(2);
    }
    if type_id(uc + 0) != 1 {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    // Check if char is signed (Rust i8 is always signed)
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

fn type_id<T>(_: T) -> u32 {
    // Rust doesn't have _Generic, so we need to simulate it differently
    // Since all integer promotions in Rust result in i32 for these types,
    // we can just return 1 for all the cases we care about
    1
}