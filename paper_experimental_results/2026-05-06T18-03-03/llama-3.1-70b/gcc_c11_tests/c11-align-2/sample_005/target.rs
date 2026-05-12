#![allow(dead_code)]

use std::mem;

const MAX_ALIGN: usize = mem::align_of::<usize>();

#[repr(align(MAX_ALIGN))]
struct AlignMaxAlignT;

#[repr(align(4))]
struct AlignInt;

#[repr(align(1))]
struct AlignChar;

#[repr(align(MAX_ALIGN))]
struct AlignMaxAlignTArray([u8; 10]);

#[repr(align(MAX_ALIGN + 0))]
struct AlignMaxAlignTPlusZero;

enum E {
    E = MAX_ALIGN,
}

#[repr(align(E::E as usize))]
struct AlignEnum;

fn func() {
    let _auto_ll: [u8; MAX_ALIGN] = [0; MAX_ALIGN];
}

fn main() {
    assert_eq!(std::any::type_name::<AlignMaxAlignT>(), "_Alignas");
    assert_eq!(std::any::type_name::<AlignInt>(), "_Alignof");
    assert_eq!(std::any::type_name::<AlignChar>(), "_Alignof");
    assert_eq!(std::any::type_name::<AlignMaxAlignTArray>(), "_Alignas");
    assert_eq!(std::any::type_name::<AlignMaxAlignTPlusZero>(), "_Alignas");
    assert_eq!(std::any::type_name::<AlignEnum>(), "_Alignas");
}