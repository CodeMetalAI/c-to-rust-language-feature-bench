fn main() -> i32 {
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = 1 * (i as i32) + (j as i32);
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let flat_index = i * 5 + j;
            let b = x.as_flattened()[flat_index];
            if a != b {
                return 1;
            }
        }
    }

    let p0 = x[0].as_ptr();
    let p1 = x[1].as_ptr();
    let diff = (p1 as isize - p0 as isize) / std::mem::size_of::<i32>() as isize;
    if diff != 5 {
        return 2;
    }

    println!("PASS");
    0
}