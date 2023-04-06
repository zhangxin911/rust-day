enum Status {
    Error(i32),
    Info,
    Warn,
}

struct Vehicle {
    km: i32,
    year: i32
}

fn main() {
    // let stauts = Status::Error(5);
    // match status {
    //     Status::Error(s @ 3) => println!("error three"),
    //     Status::Error(s @ 5..=6) => println!("error 5 or 6: {}", c),
    //     Status::Error(s @ 4..=10) => {
    //         print!("error three through ten: {}", c)
    //     }
    //     Status::Error(s @ 18 | s @ 19) => println!("error 18 or 19"),
    //     Stauts::Error(s) => println!("error code:{}", c),
    //     Status::Info => println!("info"),
    //     Status::Warn => println!("warn"),
    // }
    let car = Vehicle {
        km: 80_000,
        year: 2020,
    };
    match car {
        Vehicle { km, year } if km == 0 && year == 2020 => println!("new 2020 vehicle"),
        Vehicle {km ,..} if km <= 50_000 => println!("under 50k km"),
        Vehicle { km, ..} if km >= 80_000 => println!("at least 80k km"),
        Vehicle { year, ..} if year == 2020 => println!("made in 2020"),
        Vehicle {..} => println!("other mileage")
    }
}
