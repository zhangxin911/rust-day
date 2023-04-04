use std::collections::HashMap;
fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couch", 0);
    for (item, &qty) in stock.iter() {
        let stock_count = if qty == 0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", qty)
        };
        println!("item: {:?}, qty: {:?}", item, qty);
    }
}
