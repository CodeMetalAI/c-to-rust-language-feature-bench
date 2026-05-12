fn main() -> i32 {
    let mut i = [0; 1]; // Create an array with one element initialized to 0

    if i[0] != 0 {
        return 1;
    }
    i[0] = 7;
    if i[0] != 7 {
        return 2;
    }
    return 0;
}