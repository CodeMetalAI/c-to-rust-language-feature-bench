#[allow(non_upper_case_globals)]
#[allow(dead_code)]
fn abort() {
    panic!("abort called");
}

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
fn exit(code: i32) {
    std::process::exit(code);
}

macro_rules! CHECK_ALIGN {
    ($TYPE:ty) => {
        {
            #[repr(packed)]
            struct X {
                c: u8,
                v: $TYPE,
            }
            assert!(std::mem::align_of::<$TYPE>() <= std::mem::align_of::<X>());
        }
    };
}

fn main() {
    CHECK_ALIGN(bool);
    CHECK_ALIGN(i8);
    CHECK_ALIGN(i16);
    CHECK_ALIGN(i32);
    CHECK_ALIGN(i64);
    CHECK_ALIGN(i128);
    CHECK_ALIGN(u8);
    CHECK_ALIGN(u16);
    CHECK_ALIGN(u32);
    CHECK_ALIGN(u64);
    CHECK_ALIGN(u128);
    CHECK_ALIGN(f32);
    CHECK_ALIGN(f64);
    CHECK_ALIGN(f128);
    // Complex types are not directly supported for alignment checks
    // exit(0);
}