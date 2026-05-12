#[repr(C)]
union Bank {
    left: [i32; 8],
    right: [i32; 8],
}

static mut bank_a: Bank = Bank { left: [0; 8] };
static mut bank_b: Bank = Bank { left: [0; 8] };
static mut c: [i32; 8] = [0; 8];

fn choose_view(u: &mut Bank, which: i32) -> &[i32] {
    if which!= 0 {
        &u.right
    } else {
        &u.left
    }
}

fn fill(p: &mut [i32], base: i32) {
    for (i, elem) in p.iter_mut().enumerate() {
        *elem = base + i as i32;
    }
}

fn bump(p: &mut [i32], k: i32) {
    for elem in p.iter_mut() {
        *elem += k;
    }
}

fn sum(p: &[i32]) -> i32 {
    p.iter().sum()
}

fn main() {
    let which_a = 0;
    let which_b = 1;

    let a = choose_view(unsafe { &mut bank_a }, which_a);
    let b = choose_view(unsafe { &mut bank_b }, which_b);

    fill(a, 100);
    fill(b, 200);
    fill(&mut c, 300);

    bump(a, 1);
    bump(b, 2);
    bump(&mut c, 3);

    assert_eq!(sum(a), 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108);
    assert_eq!(sum(b), 202 + 203 + 204 + 205 + 206 + 207 + 208 + 209);
    assert_eq!(sum(&c), 303 + 304 + 305 + 306 + 307 + 308 + 309 + 310);

    assert_eq!(unsafe { bank_a.left[0] }, 101);
    assert_eq!(unsafe { bank_b.right[7] }, 209);
    assert_eq!(c[0], 303);
    assert_eq!(c[7], 310);
}