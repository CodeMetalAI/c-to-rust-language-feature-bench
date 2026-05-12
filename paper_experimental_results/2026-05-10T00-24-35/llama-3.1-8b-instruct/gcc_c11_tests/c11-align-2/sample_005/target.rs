extern crate core;

use core::mem::align_of;
use core::mem::align_up;

static mut C: [u8; 0] = [0; 0];
static mut C2: [u8; 0] = [0; 0];
static mut S: [u8; 0] = [0; 0];
static mut I: u32 = 0;
static mut L: u64 = 0;
static mut LL: u64 = 0;
static mut F: f32 = 0.0;
static mut D: f64 = 0.0;
static mut C_L_D: std::complex::Complex<f64> = std::complex::Complex::new(0.0, 0.0);
static mut CA: [u8; 10] = [0; 10];
static mut X: u32 = 0;
static mut Y: u32 = 0;
static mut S1: u64 = 0;
static mut S2: u64 = 0;

#[repr(C)]
struct EmptyStruct;

fn main() {
    let max_align = align_of::<max_align_t>();
    let ca_align = align_of::<char>();

    unsafe {
        C.align_to::<max_align_t>(core::mem::size_of::<max_align_t>() as u64);
        C2.align_to::<max_align_t>(core::mem::size_of::<max_align_t>() as u64);
        S.align_to::<max_align_t>(core::mem::size_of::<max_align_t>() as u64);

        I.align_to::<i32>(core::mem::size_of::<i32>() as u64);
        L.align_to::<max_align_t>(core::mem::size_of::<max_align_t>() as u64);
        LL.align_to::<max_align_t>(core::mem::size_of::<max_align_t>() as u64);
        F.align_to::<f32>(core::mem::size_of::<f32>() as u64);
        D.align_to::<f64>(core::mem::size_of::<f64>() as u64);
        C_L_D.align_to::<std::complex::Complex<f64>>(core::mem::size_of::<std::complex::Complex<f64>>() as u64);

        CA.align_to::<char>(core::mem::size_of::<char>() as u64);
        X.align_to::<i32>(core::mem::size_of::<i32>() as u64);
        Y.align_to::<i32>(core::mem::size_of::<i32>() as u64);

        S1.align_to::<u64>(core::mem::size_of::<u64>() as u64);
        S2.align_to::<u64>(core::mem::size_of::<u64>() as u64);

        let empty_struct = Box::<EmptyStruct>::new();
        empty_struct.align_to::<EmptyStruct>(core::mem::size_of::<EmptyStruct>() as u64);
    }

    assert_eq!(align_of::<max_align_t>(), 8);
    assert_eq!(align_of::<char>(), 1);
}