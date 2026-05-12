const TABSIZE: usize = 100;

#[allow(dead_code)]
struct Array {
    size: usize,
    data: [i32; TABSIZE],
}

fn main() {
    let array = Array {
        size: TABSIZE,
        data: [0; TABSIZE],
    };

    if array.size != TABSIZE {
        return 1;
    }

    TABSIZE = 7;
    if TABSIZE != 7 {
        return 2;
    }

    return 0;
}