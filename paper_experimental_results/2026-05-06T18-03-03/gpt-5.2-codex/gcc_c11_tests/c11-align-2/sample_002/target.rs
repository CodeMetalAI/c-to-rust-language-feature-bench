use std::process::abort;

static S1: &str = "_Alignas";
static S2: &str = "_Alignof";
static S3: &str = "1";
static S4: &str = "1";

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