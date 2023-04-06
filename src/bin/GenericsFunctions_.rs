#[derive(Debug)]
enum ServicePriority {
    High,
    Standard,
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGuset;
impl Priority for ImportantGuset {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}
#[derive(Debug)]
struct Guest;
impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

fn print_guest_priority<T: Priority + std::fmt::Debug>(guest: T) {
    println!("{:?} is {:?} priority", guest, guest.get_priority());
}

fn main() {
    let guest = Guest;
    let vip = ImportantGuset;
    print_guest_priority(guest);
    print_guest_priority(vip);
}
