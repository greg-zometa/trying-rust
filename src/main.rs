fn concat_str(a: &str, b: &str) -> String {
    return format!("{a} {b}");
}

fn add(a:i32, b:i32) -> i32 {
    return a+b;
}

fn how_big_is_num(val: i32) -> () {
    // Control flow
    if val >= 3 {
        return println!("Big num");
    } else if val == 2 {
        return println!("Medium num");
    } else if val < 2 {
        return println!("Small num");
    } else {
        return println!("Is this a number?");
    }
}

fn main() {
    // Variables are immutable by default,
    // unless specified with a `mut` keyword

    // Unless a variable is dynamically assigned,
    // Rust will implicitly infer the value type of the variable
    let one = 1;
    let two = 2; // i32
    let three = 3; // i32
    let mut mutable_num = one; // i32
    let hello = "Hello"; // &str
    let my_name = "Greg"; // &str

    let greet = concat_str(hello, my_name);
    println!("{greet:?}"); // Macro `:?` to use debug mode

    let sum = add(two, three);
    println!("{two} + {three} = {sum}"); // Macro

    how_big_is_num(one);
    how_big_is_num(two);
    how_big_is_num(three);

    while mutable_num != 5 {
        println!("{:?}", mutable_num);
        mutable_num += 1;
    }

    loop {
        if mutable_num == 1 {
            break;
        }
        println!("{:?}", mutable_num);
        mutable_num -= 1;
    }
}
