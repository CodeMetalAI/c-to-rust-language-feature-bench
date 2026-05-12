static mut I1: i32 = 1;
static mut I2: i32 = 2;
static mut I3: i32 = 3;

static mut I4: i32 = 0;
static mut I5: i32 = 0;

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

        let p_i1: *const i32 = &I1;
        let p_i2: *const i32 = &I2;
        let p_i4: *const i32 = &I4;
        let p_i5: *const i32 = &I5;

        if p_i1 != &I1 as *const i32 {
            std::process::exit(6);
        }
        if p_i2 != &I2 as *const i32 {
            std::process::exit(7);
        }
        if p_i4 != &I4 as *const i32 {
            std::process::exit(8);
        }
        if p_i5 != &I5 as *const i32 {
            std::process::exit(9);
        }

        I1 = 10;
        I2 = 20;
        I4 = 30;
        I5 = 40;

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
    }
}