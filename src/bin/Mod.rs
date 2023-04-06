mod greet {
    pub fn hello() {
        println!("hello");
    }
    fn goodbye() {
        println!("goodbye");
    }
}

fn main() {
    use greet::hello;
    use activity::math::add;
    add(1,2);
    hello();
}
