fn main() {
    const MAX: usize = 12;
    let a: [i32; MAX] = {
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
        return;
    }
    if a[1] != 3 {
        return;
    }
    if a[2] != 5 {
        return;
    }
    if a[3] != 7 {
        return;
    }
    if a[4] != 9 {
        return;
    }

    if a[MAX - 5] != 8 {
        return;
    }
    if a[MAX - 4] != 6 {
        return;
    }
    if a[MAX - 3] != 4 {
        return;
    }
    if a[MAX - 2] != 2 {
        return;
    }
    if a[MAX - 1] != 0 {
        return;
    }

    {
        let mut i = 5;
        while i < MAX - 5 {
            if a[i] != 0 {
                return;
            }
            i += 1;
        }
    }

    std::process::exit(0);
}