// function_specifiers_c8b.rs

fn _exit(status: i32) ->! {
    std::process::exit(status);
}

static mut G: i32 = 0;

fn die_if(x: i32) {
    _exit((x == 7 && unsafe { G }) as i32);
}

fn main() {
    let p = die_if;
    unsafe { G = 9 };
    p(7);
    std::process::exit(3);
}