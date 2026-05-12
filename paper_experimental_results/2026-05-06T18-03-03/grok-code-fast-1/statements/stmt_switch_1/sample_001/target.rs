fn main() {
    let expr = 0;
    {
        let mut i: i32;
        i = 4;
        match expr {
            0 => {
                i = 17;
                std::process::exit(if i == 17 { 0 } else { 1 });
            }
            _ => {}
        }
    }
}