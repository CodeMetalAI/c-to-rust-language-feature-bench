fn check_row(a: &[i32], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() -> i32 {
    let z: [i32; 12] = [1, 0, 0, 2, 0, 0, 3, 0, 0, 4, 0, 0];

    if !check_row(&z[0..3], 1, 0, 0) {
        return 1;
    }
    if !check_row(&z[3..6], 2, 0, 0) {
        return 2;
    }
    if !check_row(&z[6..9], 3, 0, 0) {
        return 3;
    }
    if !check_row(&z[9..12], 4, 0, 0) {
        return 4;
    }

    {
        let p_slice = &z[0..];
        if p_slice[1] != 0 {
            return 5;
        }
        if p_slice[4] != 2 {
            return 6;
        }
        if p_slice[7] != 0 {
            return 7;
        }
    }

    0
}