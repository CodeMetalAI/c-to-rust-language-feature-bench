fn main() {
    const TABSIZE: usize = 100;
    let a: [i32; TABSIZE] = [0; TABSIZE];

    let tabsize_size = (a.len() as usize);
    if tabsize_size != 100 {
        return 1;
    }

    let mut tabsize: i32 = 7;
    if tabsize != 7 {
        return 2;
    }

    0
}