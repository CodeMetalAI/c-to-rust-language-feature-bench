use std::mem;
use std::process;

macro_rules! check_align {
    ($type:ty) => {
        {
            struct AlignTest {
                c: u8,
                v: $type,
            }
            let x = AlignTest { c: 0, v: unsafe { mem::zeroed() } };
            let align_of_type = mem::align_of::<$type>();
            let field_offset = unsafe {
                (&x.v as *const _ as usize) - (&x.c as *const _ as usize)
            };
            // In Rust, field_offset already accounts for padding, so we can check
            // that align_of_type <= field_offset (which is the actual alignment of v).
            if align_of_type > field_offset {
                process::abort();
            }
        }
    };
}

fn main() {
    check_align!(bool);
    check_align!(i8);
    check_align!(u8);
    check_align!(i16);
    check_align!(u16);
    check_align!(i32);
    check_align!(u32);
    check_align!(i64);
    check_align!(u64);
    check_align!(i128);
    check_align!(u128);
    check_align!(isize);
    check_align!(usize);
    check_align!(f32);
    check_align!(f64);
    // Rust doesn't have long double; f64 is the widest standard float.
    // Rust doesn't have native complex types without external crates,
    // so we skip _Complex types.
    process::exit(0);
}