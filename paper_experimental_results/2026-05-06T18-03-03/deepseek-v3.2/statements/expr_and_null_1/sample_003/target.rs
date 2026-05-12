fn p(t: &mut i32) {
    *t += 1;
}

fn main() {
    let mut x =我们发现代码中有一个错误，应该是 `let mut x = 0;` 然后调用 `p(&mut x)`。
    但原C代码中 `(void)p(&x);` 是显式丢弃返回值，Rust中不需要显式 `(void)`。
    另外，原C代码的 `main` 返回 `0` 如果 `x == 1` 否则返回 `1`。
    我们需要在Rust中实现相同的逻辑。

    修正后的完整代码：
}

fn p(t: &mut i32) {
    *t += 1;
}

fn main() {
    let mut x = 0;
    p(&mut x);
    std::process::exit(if x == 1 { 0 } else { 1 });
}