fn main() {
    let nums = vec![10, 20, 30, 40];
    for num in &nums {
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
    }
    println!("number of elements = {:?}", nums.len());

    let data: Vec<_> = vec![1, 2, 3, 4, 5]
        .iter()
        .map(|value| value * 3)
        .filter(|value| value > &10)
        .collect();
}
