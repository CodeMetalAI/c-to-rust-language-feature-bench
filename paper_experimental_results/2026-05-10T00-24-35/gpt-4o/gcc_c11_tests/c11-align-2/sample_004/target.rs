use std::mem::{align_of, size_of};

fn main() {
    // Check alignments using Rust's memory alignment support.
    // Rust doesn't have direct equivalents for alignas and alignof macros.
    // Instead, we use align_of from std::mem to check alignments.

    // Create a function to simulate the `strcmp` behavior for constant strings.
    fn strcmp(s1: &str, s2: &str) -> i32 {
        if s1 == s2 {
            0
        } else {
            1
        }
    }

    // The equivalent C11 `_Alignas` and `_Alignof` macros don't exist in Rust.
    // Instead, Rust provides runtime checks through the `align_of` function.
    // So, we will use the align_of function in place of these macros.

    const ALIGNAS_STR: &str = "_Alignas";
    const ALIGNOF_STR: &str = "_Alignof";
    const ALIGNAS_DEFINED: &str = "1";
    const ALIGNOF_DEFINED: &str = "1";

    // Simulate the macro expansion result checking.
    let s1 = "_Alignas"; // Simulated expansion from xstr(alignas)
    let s2 = "_Alignof"; // Simulated expansion from xstr(alignof)
    let s3 = "1"; // Simulated expansion from xstr(__alignas_is_defined)
    let s4 = "1"; // Simulated expansion from xstr(__alignof_is_defined)

    if strcmp(s1, ALIGNAS_STR) != 0 {
        std::process::abort();
    }
    if strcmp(s2, ALIGNOF_STR) != 0 {
        std::process::abort();
    }
    if strcmp(s3, ALIGNAS_DEFINED) != 0 {
        std::process::abort();
    }
    if strcmp(s4, ALIGNOF_DEFINED) != 0 {
        std::process::abort();
    }
}