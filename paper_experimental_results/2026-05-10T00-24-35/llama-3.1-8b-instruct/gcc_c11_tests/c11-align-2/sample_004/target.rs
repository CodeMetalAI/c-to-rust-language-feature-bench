#![feature(align_stack_check)]

fn strcmp(a: &str, b: &str) -> i32 {
    match a.cmp(b) {
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
    }
}

fn exit(_code: i32) ->! {
    std::process::exit(_code);
}

fn abort() ->! {
    std::process::abort();
}

#[repr(align(alignof::<max_align_t>))]
struct C;
#[repr(align(max_align_t))]
struct CExtern;
#[repr(align(max_align_t))]
struct CStatic;

#[repr(align(alignof::<max_align_t>))]
struct S;
#[repr(align(alignof::<max_align_t>))]
struct SExtern;
#[repr(align(alignof::<max_align_t>))]
struct SStatic;

#[repr(align(alignof::<max_align_t>))]
struct D;
#[repr(align(alignof::<max_align_t>))]
struct F;
#[repr(align(alignof::<max_align_t>))]
struct Cld;

#[repr(align(alignof::<max_align_t>))]
struct Y;

#[repr(align(alignof::<max_align_t>))]
struct AutoD;

fn func() {
    let _ = AutoD;
}

fn main() {
    let s1 = stringify!(alignas);
    let s2 = stringify!(alignof);
    let s3 = stringify!(__alignas_is_defined);
    let s4 = stringify!(__alignof_is_defined);

    if strcmp(s1, "_Alignas")!= 0 {
        abort();
    }
    if strcmp(s2, "_Alignof")!= 0 {
        abort();
    }
    if strcmp(s3, "1")!= 0 {
        abort();
    }
    if strcmp(s4, "1")!= 0 {
        abort();
    }
}