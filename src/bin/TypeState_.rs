#[derive(Copy, Clone)]
struct LuggageId(usize);
struct Luggage(LuggageId);
struct CheckIn(LuggageId);
struct OnLoad(LuggageId);
struct OffLoad(LuggageId);
struct AwaitingPickUp(LuggageId);
struct EndCustBody(LuggageId);

impl Luggage {
    fn new(id: LuggageId) -> Self {
        Luggage(id)
    }
    fn check_in(self) -> CheckIn {
        CheckIn(self.0)
    }
}

impl CheckIn {
    fn onload(self) -> OnLoad {
        OnLoad(self.0)
    }
}

impl OnLoad {
    fn offload(self) -> OffLoad {
        OffLoad(self.0)
    }
}

impl OffLoad {
    fn carousel(self) -> AwaitingPickUp {
        AwaitingPickUp(self.0)
    }
}

impl AwaitingPickUp {
    fn pickup(self) -> (Luggage, EndCustBody) {
        (Luggage(self.0), EndCustBody(self.0))
    }
}

fn main() {
    let id = LuggageId(1);
    let luggage = Luggage::new(id);
    let luggage = luggage.check_in().onload().offload().carousel();
    let (luggage, _) = luggage.pickup();
}
