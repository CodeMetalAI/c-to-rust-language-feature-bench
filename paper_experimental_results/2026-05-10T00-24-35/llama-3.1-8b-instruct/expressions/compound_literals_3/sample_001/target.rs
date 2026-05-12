// Define the point struct with x and y fields.
struct Point {
  x: i32,
  y: i32,
}

// Define the foo function that takes two points and returns an integer.
fn foo(a: Point, b: Point) -> i32 {
  if a.x != 1 {
    return 1;
  }
  if a.y != 1 {
    return 2;
  }
  if b.x != 3 {
    return 3;
  }
  if b.y != 4 {
    return 4;
  }
  0
}

// Define the bar function that takes two pointer references to points and returns an integer.
fn bar(a: &Point, b: &Point) -> i32 {
  if a.x != 1 {
    return 5;
  }
  if a.y != 1 {
    return 6;
  }
  if b.x != 3 {
    return 7;
  }
  if b.y != 4 {
    return 8;
  }
  0
}

fn main() {
  let a = Point { x: 1, y: 1 };
  let b = Point { x: 3, y: 4 };
  if foo(a, b) != 0 {
    return;
  }

  let c = Point { x: 1, y: 1 };
  let d = Point { x: 3, y: 4 };
  if bar(&c, &d) != 0 {
    return;
  }

  ()
}