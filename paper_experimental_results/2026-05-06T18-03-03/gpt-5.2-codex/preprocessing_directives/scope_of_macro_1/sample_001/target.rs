static A: [i32; TABSIZE] = [0; TABSIZE];
const TABSIZE: usize = 100;

fn main() {
    if A.len() != 100 {
        std::process::exit(1);
    }

    let mut tabsize_var: i32 = 0;
    tabsize_var = 7;
    if tabsize_var != 7 {
        std::process::exit(2);
    }

    std::process::exit(0);
}