fn main() -> Result<(), i32> {
  let x = 1u32;

  let y = x << 3;
  if y != 8u32 {
    return Err(1);
  }

  if (0u32 << 5) != 0u32 {
    return Err(2);
  }

  if (2u32 << 2) != 8u32 {
    return Err(3);
  }

  Ok(())
}