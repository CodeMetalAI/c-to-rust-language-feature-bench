use std::process;

fn die_if(x: i32) ->! {
    if x == 7 && g == 9 {
        process::exit(0);
    } else {
        process::exit(2);
    }
}

fn main() {
    let p: fn(i32) ->! = die_if;
    g = 9;
    p(7);
    process::exit(3);
}

lazy_static! {
    static ref g: i32 = 9;
}