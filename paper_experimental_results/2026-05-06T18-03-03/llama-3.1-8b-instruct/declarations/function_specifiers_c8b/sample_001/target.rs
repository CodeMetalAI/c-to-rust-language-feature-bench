fn _exit(status: i32) ->! {
    std::process::exit(status);
}

fn die_if(x: i32) ->! {
    if x == 7 && g == 9 {
        _exit(0)
    } else {
        _exit(2)
    }
}

fn main() {
    let p = die_if;
    g = 9;
    p(7);
    std::process::exit(3);
}

static mut g: i32 = 0;