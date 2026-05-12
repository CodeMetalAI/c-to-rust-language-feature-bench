use std::process::exit;
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
    if get(&I1) != 1 {
        exit(1);
    }
    if get(&I2) != 2 {
        exit(2);
    }
    if get(&I3) != 3 {
        exit(3);
    }

    if get(&I4) != 0 {
        exit(4);
    }
    if get(&I5) != 0 {
        exit(5);
    }

    if !ptr::eq(P_I1, &I1) {
        exit(6);
    }
    if !ptr::eq(P_I2, &I2) {
        exit(7);
    }
    if !ptr::eq(P_I4, &I4) {
        exit(8);
    }
    if !ptr::eq(P_I5, &I5) {
        exit(9);
    }

    set(&I1, 10);
    set(&I2, 20);
    set(&I4, 30);
    set(&I5, 40);

    if get(P_I1) != 10 {
        exit(10);
    }
    if get(P_I2) != 20 {
        exit(11);
    }
    if get(P_I4) != 30 {
        exit(12);
    }
    if get(P_I5) != 40 {
        exit(13);
    }

    exit(0);
}