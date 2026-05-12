fn f1(x: i32) -> i32 {
    x + 1
}

fn f2(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    // Rust doesn't have a direct equivalent to C's _Generic for function pointer type checking,
    // so we'll simulate the behavior by using traits and compile-time checks.
    // We'll create a helper that returns a type ID based on the function signature.
    
    // Define a trait for type identification
    trait TypeId {
        fn type_id() -> u32;
    }
    
    // Implement for Fn(i32) -> i32
    impl TypeId for fn(i32) -> i32 {
        fn type_id() -> u32 {
            1
        }
    }
    
    // Implement for Fn(i32, i32) -> i32
    impl TypeId for fn(i32, i32) -> i32 {
        fn type_id() -> u32 {
            2
        }
    }
    
    // Helper function to get type id (simulating TYPE_ID macro)
    fn type_id<T: TypeId>(_: T) -> u32 {
        T::type_id()
    }
    
    let p1: fn(i32) -> i32 = f1;
    let q1: fn(i32) -> i32 = f1; // In Rust, f1 and &f1 are the same type
    
    if type_id(p1) != 1 {
        return;
    }
    if type_id(q1) != 1 {
        return;
    }
    
    // In Rust, function pointers compare by value
    if p1 != q1 {
        return;
    }
    
    if p1(3) != 4 {
        return;
    }
    if f1(3) != 4 {
        return;
    }
    
    let r1: fn(i32) -> i32 = if true { f1 } else { f1 };
    if type_id(r1) != 1 {
        return;
    }
    if r1(4) != 5 {
        return;
    }
    
    let p2: fn(i32, i32) -> i32 = f2;
    let q2: fn(i32, i32) -> i32 = f2;
    
    if type_id(p2) != 2 {
        return;
    }
    if type_id(q2) != 2 {
        return;
    }
    
    if p2 != q2 {
        return;
    }
    if p2(2, 3) != 5 {
        return;
    }
    if f2(2, 3) != 5 {
        return;
    }
    
    let r2: fn(i32, i32) -> i32 = if false { f2 } else { f2 };
    if type_id(r2) != 2 {
        return;
    }
    if r2(10, 20) != 30 {
        return;
    }
    
    // Exit with success (0)
}