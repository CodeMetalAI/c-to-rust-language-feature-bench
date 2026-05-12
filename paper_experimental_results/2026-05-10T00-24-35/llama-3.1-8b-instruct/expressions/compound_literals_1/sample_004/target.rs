struct Pair {
    a: i32,
    b: i32,
}

fn main() -> i32 {
    let p: [i32; 3] = [10, 20, 30];
    if p[0] != 10 {
        1
    } else if p[1] != 20 {
        2
    } else if p[2] != 30 {
        3
    } else {
        let q = Pair { a: 7, b: 11 };
        if q.a != 7 {
            4
        } else if q.b != 11 {
            5
        } else {
            if [1, 2, 3][1] != 2 {
                6
            } else {
                0
            }
        }
    }
}