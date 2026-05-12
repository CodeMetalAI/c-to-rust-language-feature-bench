fn main() {
    static mut I1: i32 = 1;
    static mut I2: i32 = 2;
    static mut I3: i32 = 3;
    static mut I4: i32 = 0;
    static mut I5: i32 = 0;

    static mut P_I1: *mut i32 = &mut I1 as *mut i32;
    static mut P_I2: *mut i32 = &mut I2 as *mut i32;
    static mut P_I4: *mut i32 = &mut I4 as *mut i32;
    static mut P_I5: *mut i32 = &mut I5 as *mut i32;

    if unsafe { *P_I1 }!= 1 {
        std::process::exit(1);
    }
    if unsafe { *P_I2 }!= 2 {
        std::process::exit(2);
    }
    if unsafe { I3 }!= 3 {
        std::process::exit(3);
    }

    if unsafe { *P_I4 }!= 0 {
        std::process::exit(4);
    }
    if unsafe { *P_I5 }!= 0 {
        std::process::exit(5);
    }

    if unsafe { P_I1 }!= &mut I1 as *mut i32 {
        std::process::exit(6);
    }
    if unsafe { P_I2 }!= &mut I2 as *mut i32 {
        std::process::exit(7);
    }
    if unsafe { P_I4 }!= &mut I4 as *mut i32 {
        std::process::exit(8);
    }
    if unsafe { P_I5 }!= &mut I5 as *mut i32 {
        std::process::exit(9);
    }

    unsafe {
        *P_I1 = 10;
        *P_I2 = 20;
        *P_I4 = 30;
        *P_I5 = 40;
    }

    if unsafe { *P_I1 }!= 10 {
        std::process::exit(10);
    }
    if unsafe { *P_I2 }!= 20 {
        std::process::exit(11);
    }
    if unsafe { *P_I4 }!= 30 {
        std::process::exit(12);
    }
    if unsafe { *P_I5 }!= 40 {
        std::process::exit(13);
    }

    std::process::exit(0);
}