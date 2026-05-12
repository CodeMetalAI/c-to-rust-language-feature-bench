fn f(a: i32, t: i32, c: i32) -> i32 {
    a + t + c
}

fn main() -> i32 {
    let mut a = 0;
    let mut c = 0;
    let mut t = 0;

    let v = f(a, (t = 3, t + 2), c);

    if v!= 5 {
        1
    } else {
        let mut t1 = 0;
        if (t1 = 3, t + 2)!= 5 {
            2
        } else {
            let mut t2 = 0;
            let t3 = (1 < 2) as i32 * (t2 = 0, t2 + 2) + ((1 < 2) as i32 - 1) * (t2 = 1, t2 + 2);
            if t3!= 2 {
                3
            } else {
                0
            }
        }
    }
}