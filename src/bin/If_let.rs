fn main() {
    let maybe_user = Some("Jerry");
    if let Some(user) = maybe_user {
        println!("{:?}", user);
    }
}
