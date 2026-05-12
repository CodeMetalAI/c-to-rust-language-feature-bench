// type_def_4.rs
struct Handler {
    handler: fn(i32) -> i32,
}

impl Handler {
    fn new(handler: fn(i32) -> i32) -> Self {
        Handler { handler }
    }
}

fn handler_a(x: i32) -> i32 {
    x + 1
}

fn handler_b(x: i32) -> i32 {
    x + 2
}

fn signal(s: i32, handler: &Handler) -> &dyn Fn(i32) -> i32 {
    if s & 1 {
        &handler_a
    } else {
        &handler.handler
    }
}

fn signal_alias(s: i32, handler: &Handler) -> dyn Fn(i32) -> i32 {
    signal(s, handler)
}

fn call_signal_as_plain(s: i32, handler: &dyn Fn(i32) -> i32, arg: i32) -> i32 {
    let handler = signal(s, handler);
    (handler)(arg)
}

fn call_signal_as_handler(s: i32, handler: &Handler, arg: i32) -> i32 {
    let handler = signal(s, handler);
    (handler)(arg)
}

fn call_signal_as_handler_ref(s: i32, handler: &dyn Fn(i32) -> i32, arg: i32) -> i32 {
    let handler = signal(s, handler);
    (handler)(arg)
}

fn main() {
    let mut hits = 0;

    hits = call_signal_as_plain(1, &handler_b, 3);
    if hits!= 3 + 1 {
        return;
    }

    hits = 0;
    let handler = Handler::new(handler_b);
    hits = call_signal_as_handler(2, &handler, 4);
    if hits!= 4 + 2 {
        return;
    }

    hits = 0;
    hits = call_signal_as_handler_ref(3, &handler_b, 5);
    if hits!= 5 + 1 {
        return;
    }
}