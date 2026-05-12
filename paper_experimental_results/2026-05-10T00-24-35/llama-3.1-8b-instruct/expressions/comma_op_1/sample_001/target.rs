fn f(a: i32, t: i32, c: i32) -> i32 {
    a + t + c
}

fn main() -> i32 {
    let mut a = 0;
    let mut c = 0;
    let mut t: i32 = 0;

    let v = f(a, (t = 3, t + 2), c);

    if v!= 5 {
        1
    } else {
        let t1: i32 = 0;
        if (t1 = 3, t + 2)!= 5 {
            2
        } else {
            let t2: i32 = 0;
            let t3 = if 1 < 2 { (t2 = 0, t2 + 2) } else { (t2 = 1, t2 + 2) };
            if t3!= 2 {
                3
            } else {
                0
            }
        }
    }
}