#[inline(always)]
fn check_align<T>() {
    // Create a struct with a char and a value of type T.
    struct Wrapper<T> {
        c: u8,
        v: T,
    }

    // Get the size of the type T and the alignment of the struct.
    let align_of_t = std::mem::align_of::<T>();
    let align_of_wrapper = std::mem::align_of::<Wrapper<T>>();

    // Ensure that the alignment of T is not greater than the alignment of the wrapper.
    if align_of_t > align_of_wrapper {
        std::process::abort();
    }
}

fn main() {
    check_align::<bool>();
    check_align::<char>();
    check_align::<i8>();
    check_align::<u8>();
    check_align::<i16>();
    check_align::<u16>();
    check_align::<i32>();
    check_align::<u32>();
    check_align::<i64>();
    check_align::<u64>();
    check_align::<f32>();
    check_align::<f64>();
    check_align::<f64>(); // long double is represented as f64 in Rust
    // Complex types are not directly supported in Rust's standard library.
    // Skipping complex type checks since Rust does not have built-in complex types.

    std::process::exit(0);
}