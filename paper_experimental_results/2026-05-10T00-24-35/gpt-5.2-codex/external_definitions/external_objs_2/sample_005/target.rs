use std::process::exit;
use std::sync::Mutex;

static I: Mutex<[i32; 1]> = Mutex::new([0; 1]);

fn main() {
    let mut i = I.lock().unwrap();

    if i[0] != 0 {
        exit(1);
    }
    i[0] = 7;
    if i[0] != 7 {
        exit(2);
    }
    exit(0);
}