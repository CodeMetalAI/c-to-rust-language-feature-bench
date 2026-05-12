use std::mem;

const S1: &str = std::any::type_name::<Alignas>();
const S2: &str = std::any::type_name::<Alignof>();
const S3: &str = "1";
const S4: &str = "1";

struct Alignas;
struct Alignof;

fn main() {
    if S1 != "_Alignas" {
        std::process::abort();
    }
    if S2 != "_Alignof" {
        std::process::abort();
    }
    if S3 != "1" {
        std::process::abort();
    }
    if S4 != "1" {
        std::process::abort();
    }
}