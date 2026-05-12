const TABSIZE: usize = 100;

#[allow(unused)]
fn main() {
    if a().len() != TABSIZE {
        std::process::exit(1);
    }
    TABSIZE = 7;
    if TABSIZE != 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}

fn a() -> Vec<i32> {
    vec![0; TABSIZE]
}