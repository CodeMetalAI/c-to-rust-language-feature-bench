macro_rules! type_id {
    ($x:expr) => {{
        use std::any::TypeId;
        if TypeId::of::<i32>() == TypeId::of_val(&$x) {
            1
        } else if TypeId::of::<u32>() == TypeId::of_val(&$x) {
            2
        } else {
            99
        }
    }};
}

fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = u8::MAX;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if type_id!(pc + 0) != 1 {
        std::process::exit(1);
    }
    if type_id!(sc + 0) != 1 {
        std::process::exit(2);
    }
    if type_id!(uc + 0) != 1 {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    if (pc as i32) < 0 {
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