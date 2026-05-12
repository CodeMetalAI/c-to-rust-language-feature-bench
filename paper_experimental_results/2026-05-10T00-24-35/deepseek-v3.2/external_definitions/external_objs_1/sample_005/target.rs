static mut I1: i32 = 1;
static mut I2: i32 = 2;
static mut I3: i32 = 3;

static mut I4: i32 = 0;
static mut I5: i32 = 0;

static P_I1: *mut i32 = &mut I1;
static P_I2: *mut i32 = &mut I2;
static P_I4: *mut i32 = &mut I4;
static P_I5: *mut i32 = &mut I5;

fn main() {
    if I1 != 1 {
        return println!("1");
    }
    if I2 != 2 {
        return println!("2");
    }
    if I3 != 3 {
        return println!("3");
    }

    if I4 != 0 {
        return println!("4");
    }
    if I5 != 0 {
        return println!("5");
    }

    if P_I1 != &mut I1 {
        return println!("6");
    }
    if P_I2 != &mut I2 {
        return println!("7");
    }
    if P_I4 != &mut I4 {
        return println!("8");
    }
    if P_I5 != &mut I5 {
        return println!("9");
    }

    I1 = 10;
    I2 = 20;
    I4 = 30;
    I5 = 40;

    unsafe {
        if *P_I1 != 10 {
            return println!("10");
        }
        if *P_I2 != 20 {
            return println!("11");
        }
        if *P_I4 != 30 {
            return println!("12");
        }
        if *P_I5 != 40 {
            return println!("13");
        }
    }

    println!("0");
}