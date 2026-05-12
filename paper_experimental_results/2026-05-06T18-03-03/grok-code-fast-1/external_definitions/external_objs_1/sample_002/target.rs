use std::sync::Mutex;
use std::process;

static I1: Mutex<i32> = Mutex::new(1);
static I2: Mutex<i32> = Mutex::new(2);
static I3: Mutex<i32> = Mutex::new(3);
static I4: Mutex<i32> = Mutex::new(0);
static I5: Mutex<i32> = Mutex::new(0);

fn main() {
    if *I1.lock().unwrap() != 1 {
        process::exit(1);
    }
    if *I2.lock().unwrap() != 2 {
        process::exit(2);
    }
    if *I3.lock().unwrap() != 3 {
        process::exit(3);
    }
    if *I4.lock().unwrap() != 0 {
        process::exit(4);
    }
    if *I5.lock().unwrap() != 0 {
        process::exit(5);
    }

    *I1.lock().unwrap() = 10;
    *I2.lock().unwrap() = 20;
    *I4.lock().unwrap() = 30;
    *I5.lock().unwrap() = 40;

    if *I1.lock().unwrap() != 10 {
        process::exit(10);
    }
    if *I2.lock().unwrap() != 20 {
        process::exit(11);
    }
    if *I4.lock().unwrap() != 30 {
        process::exit(12);
    }
    if *I5.lock().unwrap() != 40 {
        process::exit(13);
    }
}