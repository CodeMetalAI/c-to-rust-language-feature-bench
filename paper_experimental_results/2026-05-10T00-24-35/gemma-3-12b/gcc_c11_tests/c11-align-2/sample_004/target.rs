use std::cmp::Ordering;

#[repr(align(64))] // Assume max_align_t is 64 for this conversion
#[derive(Debug)]
struct s;

#[derive(Debug)]
struct ComplexLongDouble;

#[allow(dead_code)]
fn func() {
    #[repr(align(64))] // Assume max_align_t is 64 for this conversion
    let auto_ll;
}

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if strcmp(s1, s2) != Ordering::Equal {
        std::process::abort();
    }
    if strcmp(s3, s4) != Ordering::Equal {
        std::process::abort();
    }
}

fn strcmp(s1: &str, s2: &str) -> Ordering {
    s1.cmp(s2)
}