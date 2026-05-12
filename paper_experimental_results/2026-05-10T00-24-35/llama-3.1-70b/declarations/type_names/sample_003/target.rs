fn fip() -> *const i32 { &g_arr[1] as *const i32 }

static g_arr: [i32; 3] = [1, 2, 3];

fn f_plain() -> i32 { 7 }

fn f_var(x: u32, _args: ()) -> i32 { x as i32 + 3 }

fn use_vla(n: usize, p: *const [i32; n]) -> i32 { p[0] * 10 + p[n - 1] }

fn main() {
  {
    if std::mem::size_of_val(&(0i32))!= std::mem::size_of::<i32>() {
      std::process::exit(1);
    }
  }

  {
    let p: *const i32 = &g_arr[0];
    if unsafe { *p }!= 1 {
      std::process::exit(2);
    }
  }

  {
    let q: *const i32 = &[&g_arr[0], &g_arr[1], &g_arr[2]][2];
    if unsafe { *q }!= 3 {
      std::process::exit(3);
    }
  }

  {
    let pa: *const [i32; 3] = &g_arr;
    if unsafe { (*pa)[1] }!= 2 {
      std::process::exit(4);
    }
  }

  {
    let n: usize = 3;
    let r: i32 = use_vla(n, &g_arr);
    if r!= 13 {
      std::process::exit(5);
    }
  }

  {
    let pf: fn() -> *const i32 = fip;
    if unsafe { *pf() }!= 2 {
      std::process::exit(6);
    }
  }

  {
    let pf: fn() -> i32 = f_plain;
    if pf()!= 7 {
      std::process::exit(7);
    }
  }

  {
    let af: [&dyn Fn(u32) -> i32; 2] = [&f_var, &f_var];

    if af[0](4)!= 7 {
      std::process::exit(8);
    }

    if af[1](10)!= 13 {
      std::process::exit(9);
    }
  }

  std::process::exit(0);
}

fn f_var(x: u32) -> i32 {
    x as i32 + 3
}