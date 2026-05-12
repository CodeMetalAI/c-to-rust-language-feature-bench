fn chk(v: i16, e: i16) -> bool { v == e }

fn main() {
  let q: [[[i16; 2]; 3]; 4] = [[[1, 0], [0, 0], [0, 0]],
                              [[2, 3], [0, 0], [0, 0]],
                              [[4, 5], [6, 0], [0, 0]],
                              [[0, 0], [0, 0], [0, 0]]];

  if!chk(q[0][0][0], 1) { return 1; }
  if!chk(q[0][0][1], 0) { return 2; }
  if!chk(q[0][1][0], 0) { return 3; }
  if!chk(q[0][2][1], 0) { return 4; }

  if!chk(q[1][0][0], 2) { return 5; }
  if!chk(q[1][0][1], 3) { return 6; }
  if!chk(q[1][1][0], 0) { return 7; }

  if!chk(q[2][0][0], 4) { return 8; }
  if!chk(q[2][0][1], 5) { return 9; }
  if!chk(q[2][1][0], 6) { return 10; }
  if!chk(q[2][1][1], 0) { return 11; }

  if!chk(q[3][0][0], 0) { return 12; }
  if!chk(q[3][2][1], 0) { return 13; }

  {
    let p = &q[0][0][0] as *const i16;
    if!chk(unsafe { *p.offset(0) }, 1) { return 14; }
    if!chk(unsafe { *p.offset(6) }, 2) { return 15; }
    if!chk(unsafe { *p.offset(7) }, 3) { return 16; }
    if!chk(unsafe { *p.offset(12) }, 4) { return 17; }
    if!chk(unsafe { *p.offset(13) }, 5) { return 18; }
    if!chk(unsafe { *p.offset(14) }, 6) { return 19; }
  }

  std::process::exit(0);
}