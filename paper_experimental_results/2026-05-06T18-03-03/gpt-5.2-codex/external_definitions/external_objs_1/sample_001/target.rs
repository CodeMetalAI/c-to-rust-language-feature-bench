use std::sync::Mutex;

static I1: Mutex<i32> = Mutex::new(1);
static I2: Mutex<i32> = Mutex::new(2);
static I3: Mutex<i32> = Mutex::new(3);
static I4: Mutex<i32> = Mutex::new(0);
static I5: Mutex<i32> = Mutex::new(0);

static P_I1: &Mutex<i32> = &I1;
static P_I2: &Mutex<i32> = &I2;
static P_I4: &Mutex<i32> = &I4;
static P_I5: &Mutex<i32> = &I5;

fn main() {
    let code = if *I1.lock().unwrap() != 1 {
        1
    } else if *I2.lock().unwrap() != 2 {
        2
    } else if *I3.lock().unwrap() != 3 {
        3
    } else if *I4.lock().unwrap() != 0 {
        4
    } else if *I5.lock().unwrap() != 0 {
        5
    } else if !std::ptr::eq(P_I1, &I1) {
        6
    } else if !std::ptr::eq(P_I2, &I2) {
        7
    } else if !std::ptr::eq(P_I4, &I4) {
        8
    } else if !std::ptr::eq(P_I5, &I5) {
        9
    } else {
        *I1.lock().unwrap() = 10;
        *I2.lock().unwrap() = 20;
        *I4.lock().unwrap() = 30;
        *I5.lock().unwrap() = 40;

        if *P_I1.lock().unwrap() != 10 {
            10
        } else if *P_I2.lock().unwrap() != 20 {
            11
        } else if *P_I4.lock().unwrap() != 30 {
            12
        } else if *P_I5.lock().unwrap() != 40 {
            13
        } else {
            0
        }
    };

    std::process::exit(code);
}