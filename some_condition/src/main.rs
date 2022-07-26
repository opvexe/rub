
#[cfg(some_condition)]
fn conditional_function() {
    println!("条件编译")
}

fn main() {
    conditional_function();
    println!("Hello, world!");
}

// cargo build --features some_condition