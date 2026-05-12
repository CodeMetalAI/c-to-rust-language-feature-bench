use std::sync::Mutex;

static I: Mutex<[i32; 1]> = Mutex::new([0; 1]);

fn main() {
    {
        let i = I.lock().unwrap();
        if i[0] != 0 {
            std::process::exit(1);
        }
    }
    {
        let mut i = I.lock().unwrap();
        i[0] = 7;
    }
    {
        let i = I.lock().unwrap();
        if i[0] != 7 {
            std::process::exit(2);
        }
    }
}