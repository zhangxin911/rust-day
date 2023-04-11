use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct Structure<'a>(i32, &'a str);

impl<'a> fmt::Display for Structure<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Structure is :{} {}", self.0, self.1)
    }
}

fn main() {
    let name = "Peter";
    let age = 27;
    let person = Person { name, age };
    println!("{:?}", person);
    println!("{:#?}", person);
    let structure = Structure(1, "small structure");
    println!("{}", structure)
}
