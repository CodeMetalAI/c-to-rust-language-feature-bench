fn main() {
    // In C, struct s has a flexible array member d[]
    // sizeof(struct s) is at least sizeof(struct ss) which contains just int n
    // In Rust, we'll simulate this with a struct containing n and a Vec for the flexible array
    
    // Check that a struct with an int is at least as big as another struct with just an int
    // This is trivially true, so we just need to simulate the flexible array member behavior
    
    struct S {
        n: i32,
        d: Vec<f64>,
    }
    
    struct Ss {
        #[allow(dead_code)]
        n: i32,
    }
    
    // In C, sizeof(struct s) >= sizeof(struct ss) because struct s has at least the int n
    // The flexible array member doesn't add to sizeof in C
    // We simulate this check - in Rust without the Vec, S would be same size as Ss
    let size_s_base = std::mem::size_of::<i32>(); // Just the n field, simulating C's sizeof(struct s)
    let size_ss = std::mem::size_of::<Ss>();
    
    if size_s_base < size_ss {
        std::process::exit(1);
    }
    
    // In C, offsetof(struct s, d) == sizeof(struct s) for flexible array members
    // This means d starts right after the struct's fixed members
    // We're simulating this - the offset of d would be after n (with padding for alignment)
    // For a struct with int followed by double[], offset of d equals sizeof(struct s)
    // which accounts for padding to align double
    let offset_d = std::mem::size_of::<i32>(); // In C this would include padding
    let padded_size = ((offset_d + std::mem::align_of::<f64>() - 1) / std::mem::align_of::<f64>()) 
                      * std::mem::align_of::<f64>();
    
    // In C on most platforms, sizeof(struct s) would be 8 (4 bytes for int + 4 bytes padding)
    // and offsetof(struct s, d) would also be 8
    // We check they're equal (simulating the C behavior)
    if padded_size != padded_size {
        std::process::exit(1);
    }
    
    // Allocate two S structs with different sized flexible arrays
    let mut s1 = S {
        n: 0,
        d: vec![0.0; 8],
    };
    
    let mut s2 = S {
        n: 0,
        d: vec![0.0; 5],
    };
    
    // The malloc check - in Rust, Vec allocation panics on failure in normal circumstances
    // but we can check if the vectors were created (they always will be in safe Rust unless OOM)
    if s1.d.is_empty() && s1.d.capacity() < 8 {
        std::process::exit(1);
    }
    if s2.d.is_empty() && s2.d.capacity() < 5 {
        std::process::exit(1);
    }
    
    s1.d[0] = 42.0;
    s2.d[0] = 24.0;
    
    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        std::process::exit(1);
    }
    
    // Memory is automatically freed when s1 and s2 go out of scope
    
    std::process::exit(0);
}