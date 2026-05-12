use std::process;

fn main() {
    let expr = 0;
    let i = match expr {
        0 => 17,
        _ => 0,
    };
    process::exit(if i == 17 { 0 } else { 1 });
}