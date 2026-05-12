fn use_int(x: i32) -> i32 {
    x
}

fn main() {
    const CS: (i32,) = (1,);
    let mut ncs = CS;

    let pi = &mut ncs.0;
    *pi += 10;

    let pci = &CS.0;

    if use_int(ncs.0)!= 11 {
        std::process::exit(1);
    }

    if *pci!= 1 {
        std::process::exit(2);
    }

    const A: [[i32; 3]; 2] = [[4, 5, 6], [7, 8, 9]];

    if A[0][0]!= 4 {
        std::process::exit(3);
    }
    if A[0][1]!= 5 {
        std::process::exit(4);
    }
    if A[0][2]!= 6 {
        std::process::exit(5);
    }
    if A[1][0]!= 7 {
        std::process::exit(6);
    }
    if A[1][1]!= 8 {
        std::process::exit(7);
    }
    if A[1][2]!= 9 {
        std::process::exit(8);
    }

    std::process::exit(0);
}