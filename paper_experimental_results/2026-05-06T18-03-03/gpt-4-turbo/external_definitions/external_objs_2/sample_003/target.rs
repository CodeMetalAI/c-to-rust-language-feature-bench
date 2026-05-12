fn main() {
    let mut i = vec![0; 1]; // Initialize a vector with one element set to 0

    if i[0] != 0 {
        std::process::exit(1);
    }
    i[0] = 7;
    if i[0] != 7 {
        std::process::exit(2);
    }
}