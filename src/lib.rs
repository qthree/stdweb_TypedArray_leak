#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;

use stdweb::js_export;
use stdweb::web::TypedArray;

#[js_export]
fn data() -> TypedArray<i32> {
    let vec: Vec<i32> = (0..1000000).collect();
    vec[..].into()
}
