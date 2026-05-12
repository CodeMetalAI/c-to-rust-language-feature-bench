fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let p: &mut [i32; 3] = &mut a[1];

    p[2] = 99;

    if a[1][2] != 99 {
        return 1;
    }

    if p.as_ptr() as usize - a.as_ptr() as usize != 1 {
        return 2;
    }

    println!("0");
}