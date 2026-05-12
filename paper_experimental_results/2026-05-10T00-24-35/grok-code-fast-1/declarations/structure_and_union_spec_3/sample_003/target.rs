fn main() {
    // In C, sizeof(struct s) is 4 (size of int), offset of i is 0, offset of a is 4.
    // We simulate with a Vec<i32> of size 5: index 0 is i, indices 1-4 are a.
    let mut data: Vec<i32> = vec![0; 5];

    data[0] = 7;
    data[1] = 11;
    data[4] = 22;

    if data[0] != 7 || data[1] != 11 || data[4] != 22 {
        std::process::exit(4);
    }

    std::process::exit(0);
}