fn fsize3(n: i32) -> usize {
    let b: [char; (n + 3) as usize] = ['\0'; (n + 3) as usize];
    std::mem::size_of_val(&b)
}

fn main() {
    let size = fsize3(10);

    if size != 13 {
        std::process::exit(1);
    }

    std::process::exit(0);
}