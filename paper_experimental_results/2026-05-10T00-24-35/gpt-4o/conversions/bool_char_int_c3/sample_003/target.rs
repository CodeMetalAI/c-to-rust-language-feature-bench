fn main() {
    // Declare volatile variables as mutable
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255; // -1 as unsigned char is 255

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

    // Check type identities
    if !matches!(pc as i32, 1..=i32::MAX) {
        std::process::exit(1);
    }
    if !matches!(sc as i32, 1..=i32::MAX) {
        std::process::exit(2);
    }
    if !matches!(uc as i32, 1..=i32::MAX) {
        std::process::exit(3);
    }

    // Check value of isc
    if isc != -1 {
        std::process::exit(4);
    }

    // Determine behavior for signed char comparisons
    if -1i8 < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if iuc as u32 != 255 {
            std::process::exit(6);
        }
    } else {
        if ipc as u32 != 255 {
            std::process::exit(7);
        }
        if iuc as u32 != 255 {
            std::process::exit(8);
        }
    }

    // Exit successfully
    std::process::exit(0);
}