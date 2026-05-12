fn check_row(a: &[i32; 3], x: i32, y: i32, z: i32) -> bool {
    a[0] == x && a[1] == y && a[2] == z
}

fn main() {
    let mut z = [[0; 3]; 4];
    z[0][0] = 1;
    z[1][0] = 2;
    z[2][0] = 3;
    z[3][0] = 4;

    if !check_row(&z[0], 1, 0, 0) {
        return println!("{}", 1);
    }
    if !check_row(&z[1], 2, 0, 0) {
        return println!("{}", 2);
    }
    if !check_row(&z[2], 3, 0, 0) {
        return println!("{}", 3);
    }
    if !check_row(&z[3], 4, 0, 0) {
        return println!("{}", 4);
    }

    {
        let p = &z[0][0];
        if p + 1 != &z[0][1] {
            return println!("{}", 5);
        }
        if p + 4 != &z[1][1] {
            return println!("{}", 6);
        }
        if p + 7 != &z[2][1] {
            return println!("{}", 7);
        }
        if *(&z[0][1]) != 0 {
            return println!("{}", 5);
        }
        if *(&z[1][1]) != 2 {
            return println!("{}", 6);
        }
        if *(&z[2][1]) != 0 {
            return println!("{}", 7);
        }
    }

    println!("{}", 0);
}