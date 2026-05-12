fn main() {
    let arr = [42i32, 0i32];
    let p: *const i32 = &arr[0] as *const i32;

    if p != &arr[0] as *const i32 {
        std::process::exit(1);
    }

    let p_end1: *const i32 = &arr[1] as *const i32;
    let p_end2: *const i32 = &arr[1] as *const i32;

    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    if p_end1 == p {
        std::process::exit(3);
    }
}