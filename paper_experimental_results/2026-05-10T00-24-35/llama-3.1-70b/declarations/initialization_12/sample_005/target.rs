const MAX: usize = 12;

fn main() {
    let a = [1, 3, 5, 7, 9, 0, 0, 8, 6, 4, 2, 0];

    if a[0]!= 1 {
        return std::process::exit(1);
    }
    if a[1]!= 3 {
        return std::process::exit(2);
    }
    if a[2]!= 5 {
        return std::process::exit(3);
    }
    if a[3]!= 7 {
        return std::process::exit(4);
    }
    if a[4]!= 9 {
        return std::process::exit(5);
    }

    if a[MAX - 5]!= 8 {
        return std::process::exit(6);
    }
    if a[MAX - 4]!= 6 {
        return std::process::exit(7);
    }
    if a[MAX - 3]!= 4 {
        return std::process::exit(8);
    }
    if a[MAX - 2]!= 2 {
        return std::process::exit(9);
    }
    if a[MAX - 1]!= 0 {
        return std::process::exit(10);
    }

    for i in 5..(MAX - 5) {
        if a[i]!= 0 {
            return std::process::exit(11);
        }
    }

    return std::process::exit(0);
}