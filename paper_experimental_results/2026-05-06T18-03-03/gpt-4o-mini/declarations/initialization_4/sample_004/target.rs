fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() -> i32 {
    let z: [[i32; 3]; 4] = [[1, 0, 0], [2, 0, 0], [3, 0, 0], [4, 0, 0]];

    if !check_row(&z[0], 1, 0, 0) {
        return 1;
    }
    if !check_row(&z[1], 2, 0, 0) {
        return 2;
    }
    if !check_row(&z[2], 3, 0, 0) {
        return 3;
    }
    if !check_row(&z[3], 4, 0, 0) {
        return 4;
    }

    {
        let p = &z[0][0];
        if p[1 as usize] != 0 {
            return 5;
        }
        if p[4 as usize] != 2 {
            return 6;
        }
        if p[7 as usize] != 0 {
            return 7;
        }
    }

    0
}