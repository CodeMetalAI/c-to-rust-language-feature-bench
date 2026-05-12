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
    unsafe {
        g = 9;
        die_if(7);
    }
    // This line is never reached because die_if never returns
    // However, we need to provide a return statement to make the compiler happy
    // The value returned here is arbitrary and will never be used
    std::process::exit(3);
}