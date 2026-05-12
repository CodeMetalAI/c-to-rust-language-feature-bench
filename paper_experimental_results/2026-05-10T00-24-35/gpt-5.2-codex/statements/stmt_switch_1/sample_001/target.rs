use std::process::exit;

fn main() {
    let expr = 0;
    let mut i = 0;

    match expr {
        0 => {
            i = 17;
            exit(if i == 17 { 0 } else { 1 });
        }
        _ => {}
    }
}