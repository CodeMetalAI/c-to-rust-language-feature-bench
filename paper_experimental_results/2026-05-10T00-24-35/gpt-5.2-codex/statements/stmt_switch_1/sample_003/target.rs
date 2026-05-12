use std::process::exit;

fn main() {
    let expr = 0;

    let code = match expr {
        0 => {
            let i = 17;
            if i == 17 { 0 } else { 1 }
        }
        _ => 0,
    };

    exit(code);
}