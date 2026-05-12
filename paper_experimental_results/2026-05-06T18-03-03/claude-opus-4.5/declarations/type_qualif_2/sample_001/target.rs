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
    // ncs = cs
    // We need to use a local mutable copy since we can't use unsafe
    let mut ncs_mem: i32;
    
    // Copy cs to ncs
    ncs_mem = CS.mem;
    
    // *pi += 10 (pi points to ncs.mem)
    ncs_mem += 10;
    
    // pci = &cs.mem (pci points to cs.mem, which is const)
    let pci = &CS.mem;
    
    if use_int(ncs_mem) != 11 {
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