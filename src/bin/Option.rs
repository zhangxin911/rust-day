struct Customer {
    age: Option<i32>,
    email: String,
}

struct GroceryItem {
    name: String,
    qty: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let groceris = vec![
        GroceryItem {
            name: "banana".to_owned(),
            qty: 4,
        },
        GroceryItem {
            name: "eggs".to_owned(),
            qty: 12,
        },
    ];
    for item in groceris {
        if item.name == name {
            return Some(item.qty);
        }
    }
    None
}

enum Access {
    Admin,
    User,
    Guest,
}

fn maybe_access(name: &str) -> Option<Access> {
    match name {
        "admin" => Some(Access::Admin),
        "gary" => Some(Access::User),
        _ => None,
    }
}

fn root() -> Option<Access> {
    Some(Access::Admin)
}

fn part_1() -> bool {
    maybe_access("admin").is_some()
}

fn part_2() -> Option<Access> {
    maybe_access("root")
}

fn main() {
    let mark = Customer {
        age: Some(22),
        email: "mark@example.com".to_owned(),
    };
    let becky = Customer {
        age: None,
        email: "becky@example.com".to_owned(),
    };
    match becky.age {
        // Customer{Some(age),..} =>
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("Customer age not provided"),
    }

    let a = Some(1);
    let a_is_some = a.is_some();
    let a_is_none = a.is_none();
    let a_mapped = a.map(|num| num + 1);
}
