const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct Names<'a> {
    inner: Vec<&'a str>,
}

struct Titles<'a> {
    inner: Vec<&'a str>,
}

use std::collections::HashMap;

struct Body<'a> {
    inner: HashMap<&'a str, &'a str>,
}

fn main() {
    let data: Vec<_> = MOCK_DATA.split('\n').skip(1).collect();
    let names: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(',').nth(1))
        .collect();
    for n in names.iter().take(3) {
        println!("{}", n);
    }
    let names = Names { inner: names };

    let titles: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(',').nth(4))
        .collect();
    let titles = Titles { inner: titles };
    let data = names.inner.iter().zip(titles.inner.iter());
}
