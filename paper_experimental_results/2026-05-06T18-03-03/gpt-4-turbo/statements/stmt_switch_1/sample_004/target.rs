fn main() {
    let expr = 0;
    let mut i = 4; // Initialize i here, as Rust does not allow declarations without initialization

    match expr {
        0 => {
            i = 17;
            if i == 17 {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        },
        _ => {} // Handle other cases (though in the C code, no other cases are handled)
    }
}