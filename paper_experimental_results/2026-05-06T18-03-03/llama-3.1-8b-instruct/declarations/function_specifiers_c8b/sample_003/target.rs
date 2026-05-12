fn die_if(x: i32) -> ! {
    if x == 7 && g == 9 {
        std::process::exit(0);
    } else {
        std::process::exit(2);
    }
}

fn main() {
    let g = 9;
    let p = die_if;
    p(7);
    std::process::exit(3);
}

static mut g: i32 = 0;