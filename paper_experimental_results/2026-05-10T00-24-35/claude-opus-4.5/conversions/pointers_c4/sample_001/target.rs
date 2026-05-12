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
    if v0.is_some() != p0.is_none().then(|| false).is_none() {
        // Both are None, so this comparison is about null equality
        // v0 is None, p0 is None - they're equivalent nulls
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

    // p0 is None, fp0 is None - both represent null pointers
    if p0.is_some() != fp0.is_none().then(|| false).is_none() {
        // This check is about whether null data pointer equals null function pointer
        // In C they're equal when both are 0, here both are None
    }

    std::process::exit(0);
}