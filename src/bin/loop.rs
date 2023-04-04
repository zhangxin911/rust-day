fn main() {
    // loop {
    //     println!("hello!");
    // }
    let mut i = 3;
    loop {
        println!("{:?}", i);
        i = i - 1;
        if i == 0 {
            break;
        }
    }
    let mut counter = 5;
    while counter >= 3 {
        println!("{:?}", counter);
        counter = counter - 1;
    }
}
