static mut A: [i32; 100] = [0; 100];

static mut TABSIZE: i32 = 0;

fn main() {
    unsafe {
        if A.len() != 100 {
            std::process::exit(1);
        }
        TABSIZE = 7;
        if TABSIZE != 7 {
            std::process::exit(2);
        }
    }
    std::process::exit(0);
}