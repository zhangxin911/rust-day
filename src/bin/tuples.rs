fn coordinate() -> (i32, i32) {
    (1, 7)
}

fn main() {
    let coord = (2, 3);
    println!("{:?},{:?}", coord.0, coord.1);
    let (x, y) = coord;
    let (name, age) = ("Emma", 20);

    let (x, y) = coordinate();
}
