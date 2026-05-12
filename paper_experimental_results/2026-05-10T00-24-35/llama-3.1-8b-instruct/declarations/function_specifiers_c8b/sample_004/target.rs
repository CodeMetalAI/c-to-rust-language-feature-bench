// function_specifiers_c8b.rs

fn die_if(x: i32) -> ! {
    std::process::exit((x == 7 && g == 9) as i32);
}

static mut G: i32 = 0;

fn main() {
    let p = die_if;
    unsafe { g = 9 };
    p(7);
    std::process::exit(3);
}