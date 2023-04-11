use std::fmt;

#[derive(Debug)]
struct ComplexNum {
    real: f32,
    imag: f32,
}

impl fmt::Display for ComplexNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}i", self.real, self.imag)
    }
}

#[derive(Debug)]
struct List<T>(Vec<T>);

impl<T> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "list is: {}", self)
    }
}

fn main() {
    let num = ComplexNum {
        real: 3.3,
        imag: 7.2,
    };
    println!("{:#?}", num);
    println!("{:?}", num);

    let list = List(vec![1, 2, 3, 4]);
    println!("{:#?}", list);
    println!("{:?}", list);
}
