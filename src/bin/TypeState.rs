struct Employee<State> {
    name: String,
    state: State,
}

impl<State> Employee<State> {
    fn transition<NextState>(self, state:NextState) -> Employee<NextState> {
        Employee {
            name: self.name,
            state
        }
    }
}

struct Agreement;
struct Singature;
struct Training;
struct FailedTraining {
    score: u8,
}
struct OnBoardingComplete {
    score: u8
}

impl Employee<Agreement> {
    fn new(name: &str) -> Self {
        Self {name: name.to_string(), state:Agreement}
    }
    fn read_agreement(self) -> Employee<Singature> {
        self.transition(Singature)
    }
}

impl Employee<Singature> {
    fn sign(self) -> Employee<Training> {
        self.transition(Training)
    }
}

impl Employee<Training> {
    fn train(self, score: u8) -> Result<Employee<OnBoardingComplete>, Employee<FailedTraining>> {
        if score >7 {
            Ok(self.transition(OnBoardingComplete {score}))
        } else {
            Err(self.transition(FailedTraining {score}))
        }
    }
}

fn main() {
    let employee = Employee::new("Sara");
    let onboarded = employee.read_agreement().sign().train(6);
    match onboarded {
        Ok(emp) => println!("onboarding complete"),
        Err(emp) => println!("training failed, score: {}", emp.state.score)
    }
}