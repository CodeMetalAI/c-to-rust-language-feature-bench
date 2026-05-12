use std::sync::Mutex;

static A: [i32; 100] = [0; 100];
static TABSIZE: Mutex<i32> = Mutex::new(0);

fn main() {
    if A.len() != 100 {
        std::process::exit(1);
    }

    {
        let mut ts = TABSIZE.lock().unwrap();
        *ts = 7;
    }

    {
        let ts = TABSIZE.lock().unwrap();
        if *ts != 7 {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}