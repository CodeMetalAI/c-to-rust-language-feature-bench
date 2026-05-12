fn main() {
    let mut i: Vec<i32> = vec![0];

    if i[0] != 0 {
        return 1;
    }

    i[0] = 7;

    if i[0] != 7 {
        return 2;
    }

    0
}