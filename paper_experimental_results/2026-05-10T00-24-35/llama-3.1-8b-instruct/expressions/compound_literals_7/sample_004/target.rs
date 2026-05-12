// compound_literal_7

#[derive(Debug)]
enum MaybeIncomplete<T> {
    Complete(T),
    Incomplete(Option<&'static Self>),
}

fn eval(x: &MaybeIncomplete<i32>) -> i32 {
    match x {
        MaybeIncomplete::Complete(0) => 1,
        MaybeIncomplete::Incomplete(None) => 2,
        MaybeIncomplete::Incomplete(Some(x)) if x.car().map_or(false, |y| y != &MaybeIncomplete::Incomplete(Some(&endless_zeros))) => 3,
        _ => 0,
    }
}

const ENDOGLESS_ZEROS: MaybeIncomplete<i32> = MaybeIncomplete::Incomplete(Some(&ENDOGLESS_ZEROS));

fn main() -> i32 {
    eval(&ENDOLESS_ZEROS)
}