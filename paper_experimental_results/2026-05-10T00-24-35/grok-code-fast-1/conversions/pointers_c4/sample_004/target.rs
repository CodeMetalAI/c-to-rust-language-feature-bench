fn f(x: i32) -> i32 { x }

fn main() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct NullPtr;
    
    const NULL: NullPtr = NullPtr;
    
    let p0 = NULL;
    let v0 = NULL;
    let p1 = v0;
    
    let fp0 = NULL;
    let fp1 = NULL;
    let fp2 = v0;
    
    if p0 != p1 {
        std::process::exit(1);
    }
    if p0 != NULL {
        std::process::exit(2);
    }
    
    if v0 != NULL {
        std::process::exit(3);
    }
    if v0 != p0 {
        std::process::exit(4);
    }
    
    if fp0 != fp1 {
        std::process::exit(5);
    }
    if fp1 != fp2 {
        std::process::exit(6);
    }
    if fp0 != NULL {
        std::process::exit(7);
    }
    
    if p0 != fp0 {
        std::process::exit(8);
    }
    
    std::process::exit(0);
}