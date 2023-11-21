struct Person {
    age: i32,
    name: String,
    fav_color: String
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person {
            age: 20,
            name: String::from("Georgie"),
            fav_color: String::from("grey")
        },
        Person {
            age: 21,
            name: String::from("Mikee"),
            fav_color: String::from("orange")
        },
        Person {
            age: 13,
            name: String::from("Chris"),
            fav_color: String::from("black")
        },
        Person {
            age: 8,
            name: String::from("Bobby"),
            fav_color: String::from("red")
        },
        Person {
            age: 5,
            name: String::from("Alan"),
            fav_color: String::from("blue")
        },
        Person {
            age: 40,
            name: String::from("Adam"),
            fav_color: String::from("purple")
        },
        Person {
            age: 30,
            name: String::from("Kevin"),
            fav_color: String::from("navy blue")
        },
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color)
        }
    }
}