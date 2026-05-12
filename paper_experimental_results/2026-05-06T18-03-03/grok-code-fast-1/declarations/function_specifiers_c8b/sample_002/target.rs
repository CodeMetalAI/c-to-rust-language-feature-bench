use std::sync::Mutex;

static G: Mutex<i32> = Mutex::new(0);

fn die_if(x: i32) {
    let g = *G.lock().unwrap();
    let code = if x == 7 && g == 9 { 0 } else { 2 };
    std::process::exit(code);
}

fn main() {
    let p: fn(i32) = die_if;
    {
        let mut g = G.lock().unwrap();
        *g = 9;
    }
    p(7);
}