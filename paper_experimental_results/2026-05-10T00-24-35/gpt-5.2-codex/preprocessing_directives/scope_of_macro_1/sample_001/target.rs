const TABSIZE: usize = 100;
static A: [i32; TABSIZE] = [0; TABSIZE];

fn main() {
    if std::mem::size_of_val(&A) / std::mem::size_of::<i32>() != 100 {
        std::process::exit(1);
    }
    let mut TABSIZE: i32 = 0;
    TABSIZE = 7;
    if TABSIZE != 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}