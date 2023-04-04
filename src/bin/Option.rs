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
}
