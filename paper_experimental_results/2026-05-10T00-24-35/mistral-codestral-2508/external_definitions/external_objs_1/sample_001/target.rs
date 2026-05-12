fn main() {
    let mut i1: i32 = 1;
    let mut i2: i32 = 2;
    let mut i3: i32 = 3;

    let mut i4: i32 = 0;
    let mut i5: i32 = 0;

    let p_i1: &mut i32 = &mut i1;
    let p_i2: &mut i32 = &mut i2;
    let p_i4: &mut i32 = &mut i4;
    let p_i5: &mut i32 = &mut i5;

    if i1 != 1 {
        std::process::exit(1);
    }
    if i2 != 2 {
        std::process::exit(2);
    }
    if i3 != 3 {
        std::process::exit(3);
    }

    if i4 != 0 {
        std::process::exit(4);
    }
    if i5 != 0 {
        std::process::exit(5);
    }

    if p_i1 as *mut i32 != &i1 as *const i32 as *mut i32 {
        std::process::exit(6);
    }
    if p_i2 as *mut i32 != &i2 as *const i32 as *mut i32 {
        std::process::exit(7);
    }
    if p_i4 as *mut i32 != &i4 as *const i32 as *mut i32 {
        std::process::exit(8);
    }
    if p_i5 as *mut i32 != &i5 as *const i32 as *mut i32 {
        std::process::exit(9);
    }

    *p_i1 = 10;
    *p_i2 = 20;
    *p_i4 = 30;
    *p_i5 = 40;

    if *p_i1 != 10 {
        std::process::exit(10);
    }
    if *p_i2 != 20 {
        std::process::exit(11);
    }
    if *p_i4 != 30 {
        std::process::exit(12);
    }
    if *p_i5 != 40 {
        std::process::exit(13);
    }

    std::process::exit(0);
}