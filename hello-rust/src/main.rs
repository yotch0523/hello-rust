use chapter3::something_great_async_function;
use futures::executor;

mod chapter1;
mod chapter2;
mod chapter3;

fn main() {
    executor::block_on(something_great_async_function());
}
