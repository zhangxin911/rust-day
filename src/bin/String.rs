#[derive(Debug)]
struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person {
            name: String::from("George"),
            fav_color: String::from("green"),
            age: 7,
        },
        Person {
            name: String::from("Anna"),
            fav_color: String::from("purple"),
            age: 9,
        },
    ];

    for person in &people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color);
        }
    }
    println!("people: {:?}", people);
}
