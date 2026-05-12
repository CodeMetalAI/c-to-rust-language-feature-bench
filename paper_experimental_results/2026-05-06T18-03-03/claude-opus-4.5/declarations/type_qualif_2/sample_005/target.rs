struct S {
    mem: i32,
}

const CS: S = S { mem: 1 };
static mut NCS: S = S { mem: 0 };

const A: [[i32; 3]; 2] = [[4, 5, 6], [7, 8, 9]];

fn use_int(x: i32) -> i32 {
    x
}

fn main() {
    // We need to use interior mutability pattern to avoid unsafe
    // But since the original uses global mutable state, we'll use local variables instead
    // to maintain safe Rust while preserving the behavior
    
    let cs = S { mem: 1 };
    let mut ncs = S { mem: 0 };
    
    ncs = S { mem: cs.mem };
    
    // pi = &ncs.mem; *pi += 10;
    ncs.mem += 10;
    
    // pci = &cs.mem;
    let pci = &cs.mem;
    
    if use_int(ncs.mem) != 11 {
        std::process::exit(1);
    }
    
    if *pci != 1 {
        std::process::exit(2);
    }
    
    if A[0][0] != 4 {
        std::process::exit(3);
    }
    if A[0][1] != 5 {
        std::process::exit(4);
    }
    if A[0][2] != 6 {
        std::process::exit(5);
    }
    if A[1][0] != 7 {
        std::process::exit(6);
    }
    if A[1][1] != 8 {
        std::process::exit(7);
    }
    if A[1][2] != 9 {
        std::process::exit(8);
    }
    
    std::process::exit(0);
}