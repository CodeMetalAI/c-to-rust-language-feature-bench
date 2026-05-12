static A: [i32; 100] = [0; 100];

fn main() {
    if std::mem::size_of_val(&A) / std::mem::size_of::<i32>() != 100 {
        std::process::exit(1);
    }
    let mut tabsize: i32 = 0;
    tabsize = 7;
    if tabsize != 7 {
        std::process::exit(2);
    }
}