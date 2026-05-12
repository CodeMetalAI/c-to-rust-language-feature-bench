fn _exit(status: i32) ->! {
    std::process::exit(status);
}

fn die_if(x: i32) ->! {
    _exit((x == 7 && g == 9) as i32)
}

fn main() {
    let g = 9;
    let p = die_if;
    p(7);
    std::process::exit(3);
}

static mut g: i32 = 0;