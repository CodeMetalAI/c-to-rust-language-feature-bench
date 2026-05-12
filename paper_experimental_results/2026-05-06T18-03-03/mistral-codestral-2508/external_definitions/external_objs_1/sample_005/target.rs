static mut I1: i32 = 1;
static mut I2: i32 = 2;
static mut I3: i32 = 3;

static mut I4: i32 = 0;
static mut I5: i32 = 0;

static mut P_I1: *mut i32 = &mut I1;
static mut P_I2: *mut i32 = &mut I2;
static mut P_I4: *mut i32 = &mut I4;
static mut P_I5: *mut i32 = &mut I5;

fn main() {
    unsafe {
        if I1 != 1 {
            std::process::exit(1);
        }
        if I2 != 2 {
            std::process::exit(2);
        }
        if I3 != 3 {
            std::process::exit(3);
        }

        if I4 != 0 {
            std::process::exit(4);
        }
        if I5 != 0 {
            std::process::exit(5);
        }

        if P_I1 != &mut I1 {
            std::process::exit(6);
        }
        if P_I2 != &mut I2 {
            std::process::exit(7);
        }
        if P_I4 != &mut I4 {
            std::process::exit(8);
        }
        if P_I5 != &mut I5 {
            std::process::exit(9);
        }

        I1 = 10;
        I2 = 20;
        I4 = 30;
        I5 = 40;

        if *P_I1 != 10 {
            std::process::exit(10);
        }
        if *P_I2 != 20 {
            std::process::exit(11);
        }
        if *P_I4 != 30 {
            std::process::exit(12);
        }
        if *P_I5 != 40 {
            std::process::exit(13);
        }
    }
}