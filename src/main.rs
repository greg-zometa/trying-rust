struct Neighbor {
    name: &'static str,
    borrowing: &'static str
}

fn dans_house (neighbor: &Neighbor) {
    let name = neighbor.name;
    let borrowing = neighbor.borrowing;

    println!("{name} is at Dan's house and wants to borrow {borrowing}");
}

fn evelyns_house (neighbor: &Neighbor) {
    let name = neighbor.name;
    let borrowing = neighbor.borrowing;

    println!("{name} is at Evelyn's house and wants to borrow {borrowing}");
}

fn rons_house (neighbor: &Neighbor) {
    let name = neighbor.name;
    let borrowing = neighbor.borrowing;

    println!("{name} is at Ron's house and wants to borrow {borrowing}");
}

fn main() {
    let dave = Neighbor {
        name: "Dave",
        borrowing: "sugar"
    };

    dans_house(&dave);
    evelyns_house(&dave);
    rons_house(&dave);

}