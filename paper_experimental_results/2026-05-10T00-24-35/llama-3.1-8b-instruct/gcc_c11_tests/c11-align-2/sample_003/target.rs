#[cfg(feature = "c11")]
extern crate std;

use std::cmp::Ordering::{Eq, PartialEq};

const _ALIGNAS: &str = "_Alignas";
const _ALIGNOF: &str = "_Alignof";
const _ALIGNAS_IS_DEFINED: &str = "1";
const _ALIGNOF_IS_DEFINED: &str = "1";

#[repr(transparent)]
struct MaxAlign;

fn main() {
    assert_eq!(_ALIGNAS, "_Alignas");
    assert_eq!(_ALIGNOF, "_Alignof");
    assert_eq!(_ALIGNAS_IS_DEFINED, "1");
    assert_eq!(_ALIGNOF_IS_DEFINED, "1");
}