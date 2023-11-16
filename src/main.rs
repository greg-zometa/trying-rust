fn add(num_one: i32, num_two: i32) -> i32 {
    return num_one + num_two;
}

fn display_res(res: i32) {
    println!("{:?}", res);
}

fn main() {
    let num_one = 1;
    let num_two = 2;

    let sum = add(num_one, num_two);
    display_res(sum);
}