use std::process::abort;

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";

    if s1 != "_Alignas" {
        abort();
    }
    if s2 != "_Alignof" {
        abort();
    }
    if s3 != "1" {
        abort();
    }
    if s4 != "1" {
        abort();
    }
}