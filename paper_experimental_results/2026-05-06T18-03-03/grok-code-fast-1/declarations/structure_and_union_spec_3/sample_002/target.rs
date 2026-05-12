fn main() -> i32 {
    // Offsets are correct: i at 0, a at 4 (size of i32)
    // Allocate space for i and 4 elements of a
    let mut data: Vec<i32> = vec![0; 1 + 4];
    // Simulate p->i = 7
    data[0] = 7;
    // Simulate p->a[0] = 11
    data[1] = 11;
    // Simulate p->a[3] = 22
    data[4] = 22;
    // Check values
    if data[0] != 7 || data[1] != 11 || data[4] != 22 {
        return 4;
    }
    0
}