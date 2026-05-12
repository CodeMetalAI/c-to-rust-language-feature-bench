// scope_of_macro_1.rs

const TABSIZE: usize = 100;

fn main() {
    if a().len() != 100 {
        return 1;
    }
    TABSIZE = 7;
    if TABSIZE != 7 {
        return 2;
    }
    return 0;
}

fn a() -> Vec<i32> {
    vec![0; TABSIZE]
}