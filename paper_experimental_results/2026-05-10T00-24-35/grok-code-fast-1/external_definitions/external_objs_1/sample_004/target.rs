use std::sync::Mutex;

static I1: Mutex<i32> = Mutex::new(1);
static I2: Mutex<i32> = Mutex::new(2);
static I3: Mutex<i32> = Mutex::new(3);
static I4: Mutex<i32> = Mutex::new(0);
static I5: Mutex<i32> = Mutex::new(0);

static P_I1: &'static Mutex<i32> = &I1;
static P_I2: &'static Mutex<i32> = &I2;
static P_I4: &'static Mutex<i32> = &I4;
static P_I5: &'static Mutex<i32> = &I5;

fn main() {
    if *I1.lock().unwrap() != 1 {
        std::process::exit(1);
    }
    if *I2.lock().unwrap() != 2 {
        std::process::exit(2);
    }
    if *I3.lock().unwrap() != 3 {
        std::process::exit(3);
    }
    if *I4.lock().unwrap() != 0 {
        std::process::exit(4);
    }
    if *I5.lock().unwrap() != 0 {
        std::process::exit(5);
    }

    if !std::ptr::eq(P_I1, &I1) {
        std::process::exit(6);
    }
    if !std::ptr::eq(P_I2, &I2) {
        std::process::exit(7);
    }
    if !std::ptr::eq(P_I4, &I4) {
        std::process::exit(8);
    }
    if !std::ptr::eq(P_I5, &I5) {
        std::process::exit(9);
    }

    *I1.lock().unwrap() = 10;
    *I2.lock().unwrap() = 20;
    *I4.lock().unwrap() = 30;
    *I5.lock().unwrap() = 40;

    if *P_I1.lock().unwrap() != 10 {
        std::process::exit(10);
    }
    if *P_I2.lock().unwrap() != 20 {
        std::process::exit(11);
    }
    if *P_I4.lock().unwrap() != 30 {
        std::process::exit(12);
    }
    if *P_I5.lock().unwrap() != 40 {
        std::process::exit(13);
    }

    std::process::exit(0);
}