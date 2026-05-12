// type_qualif_2.rs
const struct S {
    mem: i32,
}

struct Sncs;

const A: [[i32; 3]; 2] = [[4, 5, 6], [7, 8, 9]];

fn use_int(x: i32) -> i32 {
    x
}

fn main() {
    let mut ncs = S { mem: 1 };

    ncs.mem += 10;

    if use_int(ncs.mem)!= 11 {
        std::process::exit(1);
    }

    if ncs.mem!= 11 {
        std::process::exit(2);
    }

    for i in 0..2 {
        for j in 0..3 {
            if A[i][j]!= (i as i32 * 3 + j + 4) {
                std::process::exit(3 + i * 3 + j);
            }
        }
    }

    std::process::exit(0);
}