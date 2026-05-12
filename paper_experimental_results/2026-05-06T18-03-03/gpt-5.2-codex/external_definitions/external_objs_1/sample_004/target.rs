use std::ptr;
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

fn get(m: &Mutex<i32>) -> i32 {
    *m.lock().unwrap()
}

fn set(m: &Mutex<i32>, v: i32) {
    *m.lock().unwrap() = v;
}

fn main() {
    let code = if get(&I1) != 1 {
        1
    } else if get(&I2) != 2 {
        2
    } else if get(&I3) != 3 {
        3
    } else if get(&I4) != 0 {
        4
    } else if get(&I5) != 0 {
        5
    } else if !ptr::eq(P_I1, &I1) {
        6
    } else if !ptr::eq(P_I2, &I2) {
        7
    } else if !ptr::eq(P_I4, &I4) {
        8
    } else if !ptr::eq(P_I5, &I5) {
        9
    } else {
        set(&I1, 10);
        set(&I2, 20);
        set(&I4, 30);
        set(&I5, 40);

        if get(P_I1) != 10 {
            10
        } else if get(P_I2) != 20 {
            11
        } else if get(P_I4) != 30 {
            12
        } else if get(P_I5) != 40 {
            13
        } else {
            0
        }
    };

    std::process::exit(code);
}