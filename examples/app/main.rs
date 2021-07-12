use std::{thread, time::Duration};

// Register one or more libraries the app will
// use to find methods from.
hotreload::register! {
    lib => "../lib/target/debug"
}

// Decorate a function to reload with this macro
// pointing to a library registered in `register!`
// so that the crate knows where to find the function.
// The body of the has a fallback implementation used
// when the hot library is not available. The function
// signature and name must match that in the hot library.
#[hotreload::from(lib)]
fn print() {
    println!("fallback");
}

// fn print() {
//     // derive the function pointer signature
//     // from the parent function signature
//     #[inline(always)]
//     fn dylib_fn(print: hotreload::lib::Symbol<fn()>) {
//         print()
//     }

//     // same signature as parent function
//     #[inline(always)]
//     fn fallback_fn() {
//         println!("fallback function");
//     }

//     // use `crate::` to allow this hot function
//     // to be declared anywhere in inside of the
//     // consumer crate
//     crate::HOTRELOAD_LIB.call_or_fallback("print", dylib_fn, fallback_fn)
// }

fn wait(time: Duration) {
    HOTRELOAD_LIB.call_or_fallback(
        "wait",
        |wait: hotreload::lib::Symbol<fn(Duration)>| wait(time),
        || {
            thread::sleep(time);
        },
    )
}

// #[hotreload::from(lib)]
// fn wait(time: Duration) {
//     thread::sleep(time);
// }

fn main() {
    loop {
        print();
        wait(Duration::from_secs(10));
    }
}
