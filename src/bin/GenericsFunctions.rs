trait CheckIn {
    fn check_in(&self) {}
    fn process(&self) {}
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("checked in as pilot");
    }
    fn process(&self) {
        println!("pilot enters the cockpit");
    }
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("checked in as passenger");
    }
    fn process(&self) {
        println!("passenger enters the cockpit");
    }
}
struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("checked in as cargo");
    }
    fn process(&self) {
        println!("cargo enters the cockpit");
    }
}

fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
}

fn main() {
    let paul = Passenger;
    let kathy = Pilot;
    let cargo1 = Cargo;
    let cargo2 = Cargo;
}
