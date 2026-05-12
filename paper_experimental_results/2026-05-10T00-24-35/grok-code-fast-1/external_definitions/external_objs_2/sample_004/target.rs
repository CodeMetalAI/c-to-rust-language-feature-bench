use std::sync::Mutex;

static I: Mutex<[i32; 1]> = Mutex::new([0]);

fn main() {
    let mut guard = I.lock().unwrap();
    if guard[0] != 0 {
        std::process::exit(1);
    }
    guard[0] = 7;
    if guard[0] != 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}