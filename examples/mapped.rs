use memoize::memoize;

#[memoize(Map: arg -> String)]
fn hello(arg: &str, arg2: usize) -> bool {
    println!("{} => {}", arg, arg2);
    arg.len() % 2 == arg2
}

fn main() {
    // `hello` is only called once here.
    assert!(!hello("World", 0));
    assert!(!hello("World", 0));
    memoized_flush_hello();
}
