fn say_my_name(first_name: &str, last_name: &str) -> () {
    println!("Hello {first_name} {last_name}!");
}

fn main() {
    let first_name = "Matt";
    let last_name = "Damon";
    say_my_name(first_name, last_name)
}