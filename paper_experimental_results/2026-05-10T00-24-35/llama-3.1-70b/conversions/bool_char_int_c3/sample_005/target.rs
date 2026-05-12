fn main() {
    let pc: char = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if std::mem::size_of_val(&(pc as i32))!= std::mem::size_of_val(&1) {
        return;
    }
    if std::mem::size_of_val(&(sc as i32))!= std::mem::size_of_val(&1) {
        return;
    }
    if std::mem::size_of_val(&(uc as i32))!= std::mem::size_of_val(&1) {
        return;
    }

    if isc!= -1 {
        return;
    }

    if pc as i32 < 0 {
        if ipc!= -1 {
            return;
        }
        if iuc as u32!= 255 {
            return;
        }
    } else {
        if ipc as u32!= 255 {
            return;
        }
        if iuc as u32!= 255 {
            return;
        }
    }

    println!("Success");
}