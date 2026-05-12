fn main() {
    let pc: char = -1 as char;
    let sc: i8 = -1;
    let uc: u8 =!0; // equivalent to (unsigned char)-1

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if std::mem::size_of_val(&(pc + 0))!= std::mem::size_of::<i32>() {
        return;
    }
    if std::mem::size_of_val(&(sc + 0))!= std::mem::size_of::<i32>() {
        return;
    }
    if std::mem::size_of_val(&(uc + 0))!= std::mem::size_of::<i32>() {
        return;
    }

    if isc!= -1 {
        return;
    }

    if pc < 0 {
        if ipc!= -1 {
            return;
        }
        if (iuc as u32)!= 255 {
            return;
        }
    } else {
        if (ipc as u32)!= 255 {
            return;
        }
        if (iuc as u32)!= 255 {
            return;
        }
    }

    std::process::exit(0);
}