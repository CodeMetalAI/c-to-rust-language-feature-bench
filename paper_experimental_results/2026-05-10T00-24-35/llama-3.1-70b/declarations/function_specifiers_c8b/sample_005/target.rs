use std::process;

fn die_if(x: i32) ->! {
    if x == 7 && g == 9 {
        process::exit(0);
    } else {
        process::exit(2);
    }
}

fn main() {
    let g = 9;
    let p = die_if;
    p(7);
}