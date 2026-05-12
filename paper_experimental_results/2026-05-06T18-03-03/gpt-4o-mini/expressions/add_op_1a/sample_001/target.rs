fn main() {
    let n = 4;
    let m = 3;
    let mut a = [[0; 3]; 4];

    let mut p = &mut a[1..]; // Move the pointer to the second row
    p[0][2] = 99; // Set the value at the new pointer location

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n = p.as_ptr() as usize / std::mem::size_of::<[i32; 3]>() / std::mem::size_of::<[i32; 3]>();
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}