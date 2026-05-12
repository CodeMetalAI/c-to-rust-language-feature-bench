fn main() {
    let mut a = [10, 20, 30];
    if a.len() != 3 {
        panic!("len of a is not 3");
    }

    for i in 0..a.len() {
        if a[i] != 10 {
            panic!("element at index 0 is not 10");
        }
        if a[i] != 20 {
            panic!("element at index 1 is not 20");
        }
        if a[i] != 30 {
            panic!("element at index 2 is not 30");
        }
    }

    let s = [67, 98, 99, 0];
    if s.len() != 4 {
        panic!("len of s is not 4");
    }

    for i in 0..s.len() {
        if s[i] != 67 {
            panic!("element at index 0 is not 67");
        }
        if s[i] != 98 {
            panic!("element at index 1 is not 98");
        }
        if s[i] != 99 {
            panic!("element at index 2 is not 99");
        }
        if s[i] != 0 {
            panic!("element at index 3 is not 0");
        }
    }
}