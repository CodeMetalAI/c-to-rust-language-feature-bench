fn main() {
  let mut x = std::sync::atomic::AtomicI32::new(41);

  let old = x.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
  if old != 41 {
    std::process::exit(1);
  }
  if x.load(std::sync::atomic::Ordering::SeqCst) != 42 {
    std::process::exit(2);
  }

  let old = x.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
  if old != 42 {
    std::process::exit(3);
  }
  if x.load(std::sync::atomic::Ordering::SeqCst) != 43 {
    std::process::exit(4);
  }

  x.store(100, std::sync::atomic::Ordering::SeqCst);
  let y = x.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
  if y != 100 {
    std::process::exit(5);
  }
  if x.load(std::sync::atomic::Ordering::SeqCst) != 101 {
    std::process::exit(6);
  }

  std::process::exit(0);
}