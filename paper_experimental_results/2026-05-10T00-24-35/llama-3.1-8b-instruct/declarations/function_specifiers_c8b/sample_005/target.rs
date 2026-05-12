fn die_if(x: i32) ->! {
    std::process::exit(if x == 7 && g == 9 { 0 } else { 2 });
}

fn main() {
    let g = 9;
    let p = || die_if(7);
    p();
    std::process::exit(3);
}

static mut G: i32 = 0;