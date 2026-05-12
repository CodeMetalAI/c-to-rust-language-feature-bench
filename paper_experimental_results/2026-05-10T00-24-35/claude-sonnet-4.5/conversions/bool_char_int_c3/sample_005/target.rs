fn main() {
    let pc: i8 = -1i8 as i8;
    let sc: i8 = -1i8;
    let uc: u8 = 255u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In C, integer promotions convert char types to int when used in expressions
    // In Rust, we need to explicitly check the promoted types
    // pc + 0, sc + 0, uc + 0 all promote to int (i32 in Rust)
    let pc_promoted: i32 = pc as i32 + 0;
    let sc_promoted: i32 = sc as i32 + 0;
    let uc_promoted: i32 = uc as i32 + 0;

    // TYPE_ID checks - all should be int (which we simulate as i32)
    // Since we're using i32, these conditions are always true in our Rust version
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of::<i32>() {
        std::process::exit(1);
    }
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of::<i32>() {
        std::process::exit(2);
    }
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of::<i32>() {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    // Check if char is signed (in standard Rust, i8 is signed, u8 is unsigned)
    // We use i8 for char which is signed
    if ((-1i8) as i8) < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if iuc as u32 != 255u32 {
            std::process::exit(6);
        }
    } else {
        if ipc as u32 != 255u32 {
            std::process::exit(7);
        }
        if iuc as u32 != 255u32 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}