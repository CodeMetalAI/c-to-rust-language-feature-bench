struct V {
    data: Data,
    m: i32,
}

union Data {
    int_pair: IntPair,
    long_pair: LongPair,
}

#[derive(Debug)]
struct IntPair {
    i: i32,
    j: i32,
}

#[derive(Debug)]
struct LongPair {
    k: i64,
    l: i64,
}

fn main() {
    let mut v1 = V {
        data: Data {
            int_pair: IntPair { i: 0, j: 0 },
        },
        m: 0,
    };

    unsafe {
        v1.data.int_pair.i = 2;
        v1.data.long_pair.k = 5;

        if v1.data.int_pair.i != 2 {
            std::process::exit(1);
        }

        if v1.data.long_pair.k != 5 {
            std::process::exit(1);
        }
    }
}