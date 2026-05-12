fn main() {
    const MAX: usize = 12;
    
    // C designated initializer: {1, 3, 5, 7, 9, [MAX - 5] = 8, 6, 4, 2, 0}
    // This sets indices 0-4 to 1,3,5,7,9, then jumps to index 7 (MAX-5) and sets 8,6,4,2,0
    // Indices 5 and 6 are implicitly 0
    let a: [i32; MAX] = [1, 3, 5, 7, 9, 0, 0, 8, 6, 4, 2, 0];

    if a[0] != 1 {
        std::process::exit(1);
    }
    if a[1] != 3 {
        std::process::exit(2);
    }
    if a[2] != 5 {
        std::process::exit(3);
    }
    if a[3] != 7 {
        std::process::exit(4);
    }
    if a[4] != 9 {
        std::process::exit(5);
    }

    if a[MAX - 5] != 8 {
        std::process::exit(6);
    }
    if a[MAX - 4] != 6 {
        std::process::exit(7);
    }
    if a[MAX - 3] != 4 {
        std::process::exit(8);
    }
    if a[MAX - 2] != 2 {
        std::process::exit(9);
    }
    if a[MAX - 1] != 0 {
        std::process::exit(10);
    }

    {
        let mut i = 5;
        while i < MAX - 5 {
            if a[i] != 0 {
                std::process::exit(11);
            }
            i += 1;
        }
    }

    std::process::exit(0);
}