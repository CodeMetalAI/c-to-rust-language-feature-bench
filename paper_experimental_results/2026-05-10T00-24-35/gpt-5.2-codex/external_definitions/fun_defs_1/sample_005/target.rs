use std::sync::Mutex;

static ARR: Mutex<[i32; 256]> = Mutex::new([0; 256]);

fn f(_a: u8, b: u8) -> i32 {
    let mut arr = ARR.lock().unwrap();
    arr[b as usize] = 1;
    b as i32
}

fn g(a: i32, b: i32) -> i32 {
    let mut arr = ARR.lock().unwrap();
    let idx = (b as u8) as usize;
    arr[idx] += b;
    a + b
}

fn main() {
    let a: i8 = 0;
    let b: i8 = -1;

    let r1 = f(a as u8, b as u8);
    let r2 = g(a as i32, b as i32);

    let code = if r1 != 255 {
        1
    } else if r2 != -1 {
        2
    } else {
        let ub = b as u8;
        let val = {
            let arr = ARR.lock().unwrap();
            arr[ub as usize]
        };
        if val != 0 { 3 } else { 0 }
    };

    std::process::exit(code);
}