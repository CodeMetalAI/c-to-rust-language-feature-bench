fn check_row(a: [i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let z: [[i32; 3]; 4] = [[1], [2], [3], [4]];

    if!check_row(z[0], 1, 0, 0) {
        return;
    }
    if!check_row(z[1], 2, 0, 0) {
        return;
    }
    if!check_row(z[2], 3, 0, 0) {
        return;
    }
    if!check_row(z[3], 4, 0, 0) {
        return;
    }

    {
        let p = &z[0][0];
        if p[1]!= 0 {
            return;
        }
        if p[4]!= 2 {
            return;
        }
        if p[7]!= 0 {
            return;
        }
    }

    ()
}