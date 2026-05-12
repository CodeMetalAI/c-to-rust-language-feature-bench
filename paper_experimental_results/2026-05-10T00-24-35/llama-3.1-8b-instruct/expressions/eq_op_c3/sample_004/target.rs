fn main() -> i32 {
    let mut a = 1;
    let mut b = 2;
    let mut c = 3;
    let mut d = 4;

    let result;
    if (a < b && c < d)!= 1 {
        result = 1;
    } else {
        a = 2;
        b = 1;
        c = 4;
        d = 3;
        if (a < b && c < d)!= 1 {
            result = 2;
        } else {
            a = 1;
            b = 2;
            c = 4;
            d = 3;
            if (a < b && c < d)!= 0 {
                result = 3;
            } else {
                a = 2;
                b = 1;
                c = 3;
                d = 4;
                if (a < b && c < d)!= 0 {
                    result = 4;
                } else {
                    if (a < b && c < d)!= 1 {
                        result = 5;
                    } else {
                        result = 0;
                    }
                }
            }
        }
    }
    result
}