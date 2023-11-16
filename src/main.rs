fn concat_str(a: &str, b: &str) -> String {
    return format!("{a} {b}");
}

fn add(a:i32, b:i32) -> i32 {
    return a+b;
}

fn main() {
    // Variables are immutable by default,
    // unless specified with a `mut` keyword

    // Unless a variable is dynamically assigned,
    // Rust will implicitly infer the value type of the variable
    let two = 2; // i32
    let three = 3; // i32
    let hello = "Hello"; // &str
    let my_name = "Greg";

    let greet = concat_str(hello, my_name);
    println!("{greet}"); // Macro

    let sum = add(two, three);
    println!("{two} + {three} = {sum}") // Macro
}
