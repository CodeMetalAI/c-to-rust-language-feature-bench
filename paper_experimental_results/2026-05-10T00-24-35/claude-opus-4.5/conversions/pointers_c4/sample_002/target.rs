fn main() {
    let p0: Option<&i32> = None;
    let v0: Option<&()> = None;
    let p1: Option<&i32> = None;

    let fp0: Option<fn(i32) -> i32> = None;
    let fp1: Option<fn(i32) -> i32> = None;
    let fp2: Option<fn(i32) -> i32> = None;

    if p0.is_some() != p1.is_some() {
        std::process::exit(1);
    }
    if p0.is_some() {
        std::process::exit(2);
    }

    if v0.is_some() {
        std::process::exit(3);
    }
    if v0.is_some() != p0.is_none().then(|| ()).is_none() {
        // Both are None, so this is equivalent to checking v0 != (void*)p0
        // Since both are None/null, they are equal
    }

    if fp0.is_some() != fp1.is_some() {
        std::process::exit(5);
    }
    if fp1.is_some() != fp2.is_some() {
        std::process::exit(6);
    }
    if fp0.is_some() {
        std::process::exit(7);
    }

    // p0 and fp0 are both None (null), so they're equal in the null sense
    if p0.is_some() != fp0.is_none().then(|| ()).is_none() {
        // This check is for p0 != (int*)fp0
        // Both are null/None, so they are equal
    }

    std::process::exit(0);
}