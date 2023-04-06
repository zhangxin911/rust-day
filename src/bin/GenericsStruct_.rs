trait Body {}
trait Color {}

struct Vehicle<B: Body, C: Color> {
    body: B,
    color: C,
}

impl<B: Body, C: Color> Vehicle<B, C> {
    pub fn new(body: B, color: C) -> Self {
        Self { body, color }
    }
}

struct Car;
impl Body for Car {}
impl Color for Car {}
struct Truck;
impl Body for Truck {}

struct Red;
impl Color for Red{}
struct Blue;
impl Color for Blue{}
fn main() {
    let red_truck = Vehicle::new(Truck
        , Red);
    let blue_car = Vehicle::new(Car,Blue);
}
