#[macro_use]
extern crate debug_rs;


fn main() {
    debug!(666, 33, "aaa");

    debug!(vec![1, 2, 3]);
}
