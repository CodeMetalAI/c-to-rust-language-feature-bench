fn main() {
    const MAX: usize = 12;
    let a = {
        let mut arr = [0; MAX];
        arr[0] = 1;
        arr[1] = 3;
        arr[2] = 5;
        arr[3] = 7;
        arr[4] = 9;
        arr[MAX - 5] = 8;
        arr[MAX - 4] = 6;
        arr[MAX - 3] = 4;
        arr[MAX - 2] = 2;
        arr[MAX - 1] = 0;
        arr
    };

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

    let mut i = 5;
    while i < MAX - 5 {
        if a[i] != 0 {
            std::process::exit(11);
        }
        i += 1;
    }

    std::process::exit(0);
}