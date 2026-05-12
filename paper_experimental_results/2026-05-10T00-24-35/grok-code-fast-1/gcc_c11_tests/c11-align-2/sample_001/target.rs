#[repr(align(1))]
struct S;

fn main() {
    const S1: &str = "_Alignas";
    const S2: &str = "_Alignof";
    const S3: &str = "1";
    const S4: &str = "1";
    if S1 != "_Alignas" {
        panic!();
    }
    if S2 != "_Alignof" {
        panic!();
    }
    if S3 != "1" {
        panic!();
    }
    if S4 != "1" {
        panic!();
    }
}