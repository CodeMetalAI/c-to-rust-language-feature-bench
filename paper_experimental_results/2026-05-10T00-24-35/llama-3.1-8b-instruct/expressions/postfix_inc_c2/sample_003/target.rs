use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
  let mut x = AtomicI32::new(41);

  let old = x.fetch_add(1, Ordering::SeqCst);
  if old!= 41 {
    panic!("old!= 41");
  }
  if x.load(Ordering::SeqCst)!= 42 {
    panic!("x!= 42");
  }

  let old = x.fetch_add(1, Ordering::SeqCst);
  if old!= 42 {
    panic!("old!= 42");
  }
  if x.load(Ordering::SeqCst)!= 43 {
    panic!("x!= 43");
  }

  x.store(100, Ordering::SeqCst);
  let y = x.fetch_add(1, Ordering::SeqCst) + 0;
  if y!= 100 {
    panic!("y!= 100");
  }
  if x.load(Ordering::SeqCst)!= 101 {
    panic!("x!= 101");
  }
}