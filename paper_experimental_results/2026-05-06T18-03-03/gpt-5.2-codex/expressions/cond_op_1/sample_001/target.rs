#[derive(Clone, Copy)]
struct ConstVoidPtr;
#[derive(Clone, Copy)]
struct VoidPtr;
#[derive(Clone, Copy)]
struct ConstIntPtr;
#[derive(Clone, Copy)]
struct VolatileIntPtr;
#[derive(Clone, Copy)]
struct ConstVolatileIntPtr;
#[derive(Clone, Copy)]
struct IntPtr;
#[derive(Clone, Copy)]
struct ConstCharPtr;
#[derive(Clone, Copy)]
struct NullPtr;

const C_VP: ConstVoidPtr = ConstVoidPtr;
const VP: VoidPtr = VoidPtr;
const C_IP: ConstIntPtr = ConstIntPtr;
const V_IP: VolatileIntPtr = VolatileIntPtr;
const IP: IntPtr = IntPtr;
const C_CP: ConstCharPtr = ConstCharPtr;
const NULL_PTR: NullPtr = NullPtr;

impl From<ConstIntPtr> for ConstVoidPtr {
    fn from(_: ConstIntPtr) -> Self {
        ConstVoidPtr
    }
}
impl From<VoidPtr> for ConstVoidPtr {
    fn from(_: VoidPtr) -> Self {
        ConstVoidPtr
    }
}
impl From<ConstCharPtr> for ConstVoidPtr {
    fn from(_: ConstCharPtr) -> Self {
        ConstVoidPtr
    }
}

impl From<NullPtr> for VolatileIntPtr {
    fn from(_: NullPtr) -> Self {
        VolatileIntPtr
    }
}

impl From<ConstIntPtr> for ConstVolatileIntPtr {
    fn from(_: ConstIntPtr) -> Self {
        ConstVolatileIntPtr
    }
}
impl From<VolatileIntPtr> for ConstVolatileIntPtr {
    fn from(_: VolatileIntPtr) -> Self {
        ConstVolatileIntPtr
    }
}

impl From<IntPtr> for ConstIntPtr {
    fn from(_: IntPtr) -> Self {
        ConstIntPtr
    }
}

impl From<IntPtr> for VoidPtr {
    fn from(_: IntPtr) -> Self {
        VoidPtr
    }
}

fn f(p: ConstVoidPtr) {
    let _ = p;
}
fn f1(p: VoidPtr) {
    let _ = p;
}
fn f2(p: VolatileIntPtr) {
    let _ = p;
}
fn f4(p: ConstIntPtr) {
    let _ = p;
}
fn f3(p: ConstVolatileIntPtr) {
    let _ = p;
}

fn cond<T, A: Into<T>, B: Into<T>>(c: bool, a: A, b: B) -> T {
    if c { a.into() } else { b.into() }
}

fn main() {
    f(cond(true, C_VP, C_IP));
    f(cond(true, C_IP, C_VP));

    f2(cond(true, V_IP, NULL_PTR));
    f2(cond(true, NULL_PTR, V_IP));

    f3(cond(true, C_IP, V_IP));
    f3(cond(true, V_IP, C_IP));

    f(cond(true, VP, C_CP));
    f(cond(true, C_CP, VP));

    f4(cond(true, IP, C_IP));
    f4(cond(true, C_IP, IP));

    f1(cond(true, VP, IP));
    f1(cond(true, IP, VP));
}