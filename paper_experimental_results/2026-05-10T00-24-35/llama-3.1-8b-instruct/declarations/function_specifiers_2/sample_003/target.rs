#![no_main]
#![no_std]

use panic_halt as _;

use core::hint::unreachable_unchecked;

#[no_mangle]
fn stop() -> ! {
  loop {
    let mut x: u8 = 0;
    x += 1;
  }
}

#[no_mangle]
fn f() -> ! {
  stop()
}

#[no_mangle]
fn g(i: u32) -> ! {
  if i > 0 {
    stop()
  }
}

fn main() -> ! {
  f();

  g(1);

  unreachable_unchecked()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  loop {}
}