use std::process;

const S1: &str = "_Alignas";
const S2: &str = "_Alignof";
const S3: &str = "1";
const S4: &str = "1";

fn main() {
    if S1 != "_Alignas" {
        process::abort();
    }
    if S2 != "_Alignof" {
        process::abort();
    }
    if S3 != "1" {
        process::abort();
    }
    if S4 != "1" {
        process::abort();
    }
}