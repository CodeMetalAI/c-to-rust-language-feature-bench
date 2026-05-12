static mut i1: i32 = 1;
static mut i2: i32 = 2;
extern "C" {
    static mut i3: i32;
}

static mut i4: i32 = 0;
static mut i5: i32 = 0;

fn main() {
    if unsafe { i1 }!= 1 {
        std::process::exit(1);
    }
    if unsafe { i2 }!= 2 {
        std::process::exit(2);
    }
    if unsafe { i3 }!= 3 {
        std::process::exit(3);
    }

    if unsafe { i4 }!= 0 {
        std::process::exit(4);
    }
    if unsafe { i5 }!= 0 {
        std::process::exit(5);
    }

    let p_i1 = &mut i1 as *mut i32;
    let p_i2 = &mut i2 as *mut i32;
    let p_i4 = &mut i4 as *mut i32;
    let p_i5 = &mut i5 as *mut i32;

    if p_i1 as usize!= &mut i1 as *mut i32 as usize {
        std::process::exit(6);
    }
    if p_i2 as usize!= &mut i2 as *mut i32 as usize {
        std::process::exit(7);
    }
    if p_i4 as usize!= &mut i4 as *mut i32 as usize {
        std::process::exit(8);
    }
    if p_i5 as usize!= &mut i5 as *mut i32 as usize {
        std::process::exit(9);
    }

    unsafe {
        i1 = 10;
        i2 = 20;
        i4 = 30;
        i5 = 40;
    }

    if unsafe { *p_i1 }!= 10 {
        std::process::exit(10);
    }
    if unsafe { *p_i2 }!= 20 {
        std::process::exit(11);
    }
    if unsafe { *p_i4 }!= 30 {
        std::process::exit(12);
    }
    if unsafe { *p_i5 }!= 40 {
        std::process::exit(13);
    }

    std::process::exit(0);
}