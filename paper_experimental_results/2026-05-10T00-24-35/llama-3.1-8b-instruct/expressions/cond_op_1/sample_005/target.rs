fn f<T: Copy>(p: &T) {}
fn f1<T>(p: &T) {}
fn f2<T: Copy>(p: &T) {}
fn f3<T: Copy>(p: &T) {}
fn f4<T: Copy>(p: &T) {}

fn main() {
    f(1 && c_vp as *const _ as *const _);
    f(1 && c_ip as *const _ as *const _);

    f2(1 && v_ip as *const _ as *const _);
    f2(1 && 0 as *const _ as *const _);

    f3(1 && c_ip as *const _ as *const _);
    f3(1 && v_ip as *const _ as *const _);

    f(1 && vp as *const _ as *const _);
    f(1 && c_cp as *const _ as *const _);

    f4(1 && ip as *const _ as *const _);
    f4(1 && c_ip as *const _ as *const _);

    f1(1 && vp as *const _ as *const _);
    f1(1 && ip as *const _ as *const _);

    // Note: Rust does not have a direct equivalent to C's volatile keyword.
    // This example uses a wrapper struct to simulate volatile behavior.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct VolatileWrapper<T>(pub T);
    unsafe impl<T: Copy> std::ops::Deref for VolatileWrapper<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let v_ip = VolatileWrapper(5);
    let c_ip = 5;

    f2(1 && v_ip as *const _ as *const _);
    f2(1 && 0 as *const _ as *const _);

    f3(1 && c_ip as *const _ as *const _);
    f3(1 && v_ip as *const _ as *const _);
}