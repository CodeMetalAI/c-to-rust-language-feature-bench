fn check_row(a: [i32; 3], x: i32, y: i32, z: i32) -> bool {
    a == [x, y, z]
}

fn main() {
    let y1 = [
        [1, 3, 5],
        [2, 4, 6],
        [3, 5, 7],
        [0, 0, 0],
    ];

    let y2 = [1, 3, 5, 2, 4, 6, 3, 5, 7];

    if!check_row(y1[0], 1, 3, 5) {
        println!("Error 1");
        return;
    }
    if!check_row(y1[1], 2, 4, 6) {
        println!("Error 2");
        return;
    }
    if!check_row(y1[2], 3, 5, 7) {
        println!("Error 3");
        return;
    }
    if!check_row(y1[3], 0, 0, 0) {
        println!("Error 4");
        return;
    }

    if!check_row(y2[0..3].try_into().unwrap(), 1, 3, 5) {
        println!("Error 5");
        return;
    }
    if!check_row(y2[3..6].try_into().unwrap(), 2, 4, 6) {
        println!("Error 6");
        return;
    }
    if!check_row(y2[6..9].try_into().unwrap(), 3, 5, 7) {
        println!("Error 7");
        return;
    }
    if!check_row(y2[9..12].try_into().unwrap(), 0, 0, 0) {
        println!("Error 8");
        return;
    }

    let p1 = &y1[0];
    let p2 = &y2[0..9].collect::<Vec<_>>()[..];
    if p1[11]!= 0 {
        println!("Error 9");
        return;
    }
    if p2[11]!= 0 {
        println!("Error 10");
        return;
    }

    println!("Success");
}