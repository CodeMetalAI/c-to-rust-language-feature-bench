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
    let g = 9;
    let p = die_if;
    p(7);
    std::process::exit(3);
}