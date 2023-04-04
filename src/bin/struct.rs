struct GroceryItem {
    stock: i32,
    price: f64,
}

enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("Sparkling"),
        Flavor::Sweet => println!("Sweet"),
        Flavor::Fruity => println!("Fruity"),
    }
    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };
    println!("{:?}", cereal.stock);

    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 6.0
    };
}
