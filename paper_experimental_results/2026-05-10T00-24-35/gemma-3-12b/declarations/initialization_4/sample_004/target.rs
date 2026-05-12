fn check_row(a: &[int; 3], x: int, y: int, z: int) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let z: [[int; 3]; 4] = [[1], [2], [3], [4]];

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
        let mut p: Vec<int> = Vec::new();
        p.extend_from_slice(&z[0].iter().cloned().collect::<Vec<int>>());
        if p[1] != 0 {
            return 5;
        }
        if p[4] != 2 {
            return 6;
        }
        if p[7] != 0 {
            return 7;
        }
    }

    0
}