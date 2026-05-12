fn main() {
    let mut x = 7;
    let p = &x as *const i32;

    if &x as *const i32 != p {
        std::process::exit(1);
    }

    if &(*p) as *const i32 != p {
        std::process::exit(2);
    }

    let a = [10, 20, 30];

    if &a[0] as *const i32 != &a as *const [i32] as *const i32 + 0 {
        std::process::exit(3);
    }
    if &a[1] as *const i32 != &a as *const [i32] as *const i32 + 1 {
        std::process::exit(4);
    }
    if &a[2] as *const i32 != &a as *const [i32] as *const i32 + 2 {
        std::process::exit(5);
    }

    if (a.as_ptr() + 1).read_unaligned(..) != 20 {
        std::process::exit(6);
    }

    std::process::exit(0);
}