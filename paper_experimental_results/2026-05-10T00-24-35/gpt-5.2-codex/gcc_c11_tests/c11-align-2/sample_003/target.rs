use std::process::abort;

const S1: &str = "_Alignas";
const S2: &str = "_Alignof";
const S3: &str = "1";
const S4: &str = "1";

fn main() {
    if S1 != "_Alignas" {
        abort();
    }
    if S2 != "_Alignof" {
        abort();
    }
    if S3 != "1" {
        abort();
    }
    if S4 != "1" {
        abort();
    }
}