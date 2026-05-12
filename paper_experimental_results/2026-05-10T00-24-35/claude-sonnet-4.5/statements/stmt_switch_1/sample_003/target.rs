fn main() {
    let expr = 0;

    match expr {
        0 => {
            let i: i32;
            i = 17;
            std::process::exit(if i == 17 { 0 } else { 1 });
        }
        _ => {}
    }
}