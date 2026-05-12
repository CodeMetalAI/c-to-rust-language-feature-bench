use std::process;

fn main() {
    let s1: &str = "_Alignas";
    let s2: &str = "_Alignof";
    let s3: &str = "1";
    let s4: &str = "1";

    if s1 != "_Alignas" {
        process::abort();
    }
    if s2 != "_Alignof" {
        process::abort();
    }
    if s3 != "1" {
        process::abort();
    }
    if s4 != "1" {
        process::abort();
    }
}