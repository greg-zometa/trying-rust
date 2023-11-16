fn main() {
    let match_box = "matches";
    match match_box {
        "matches" => println!("Box full of matches"),
        "few matches" => println!("Theres some matches"),
        _ => println!("Nota match box")
    }
}