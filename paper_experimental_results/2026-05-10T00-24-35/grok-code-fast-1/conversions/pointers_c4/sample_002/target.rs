fn main() {
    type NullPtr = usize;
    const NULL: NullPtr = 0;

    let p0: NullPtr = NULL;
    let v0: NullPtr = NULL;
    let p1: NullPtr = v0;

    let fp0: NullPtr = NULL;
    let fp1: NullPtr = NULL;
    let fp2: NullPtr = v0;

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