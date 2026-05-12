use std::process;

fn die_if(x: i32) ->! {
    if x == 7 && g == 9 {
        process::exit(0);
    } else {
        process::exit(2);
    }
}

static mut g: i32 = 0;

fn main() {
    let p = die_if;
    unsafe { g = 9; }
    p(7);
    // This line will never be reached due to the _Exit in die_if
    // However, we include it to match the original C code's behavior
    // If die_if does not exit, this line will be executed
    std::process::exit(3);
}