fn type_id<T: std::convert::TryFrom<i8> + std::convert::TryFrom<u8>>(x: T) -> u8 {
    match x {
        _ if T::try_from(-1).unwrap() >= 0 => 1,
        _ if T::try_from(255u8).unwrap() <= 0 => 2,
        _ => 3,
    }
}

fn main() {
    let pc = -1i8 as i8;
    let sc = -1i8 as i8;
    let uc = -1i8 as i8;

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

    if type_id(pc + 0)!= 1 {
        println!("Return code 1");
        return;
    }
    if type_id(sc + 0)!= 1 {
        println!("Return code 2");
        return;
    }
    if type_id(uc + 0)!= 1 {
        println!("Return code 3");
        return;
    }

    if isc!= -1 {
        println!("Return code 4");
        return;
    }

    if -1i8 < 0 {
        if ipc!= -1 {
            println!("Return code 5");
            return;
        }
        if (255u8 as i8) as i32!= iuc {
            println!("Return code 6");
            return;
        }
    } else {
        if (255u8 as i8) as i32!= ipc {
            println!("Return code 7");
            return;
        }
        if (255u8 as i8) as i32!= iuc {
            println!("Return code 8");
            return;
        }
    }

    println!("Return code 0");
}