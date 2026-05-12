trait TypeId {
    fn get_type_id(&self) -> i32;
}

impl TypeId for i32 {
    fn get_type_id(&self) -> i32 { 1 }
}

impl TypeId for u32 {
    fn get_type_id(&self) -> i32 { 2 }
}

impl<T> TypeId for T {
    fn get_type_id(&self) -> i32 { 99 }
}

fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if (pc + 0).get_type_id() != 1 {
        std::process::exit(1);
    }
    if (sc + 0).get_type_id() != 1 {
        std::process::exit(2);
    }
    if (uc + 0).get_type_id() != 1 {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    let is_char_signed = pc < 0;
    if is_char_signed {
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

    std::process::exit(0);
}