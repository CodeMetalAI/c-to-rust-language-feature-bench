fn check_row(a: &[int; 3], x: int, y: int, z: int) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let z = [
        [1],
        [2],
        [3],
        [4],
    ];

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
        let mut p = &z[0][0];
        if p[0] != 0 {
            return 5;
        }
        if z[1][0] != 2 {
            return 6;
        }
        if z[3][0] != 0 {
            return 7;
        }
    }

    0
}