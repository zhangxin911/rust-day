trait Sale {
    fn amount(&self) -> f64;
}

struct FullSale(f64);
impl Sale for FullSale {
    fn amount(&self) -> f64 {
        self.0
    }
}
struct OneDollarOffCoupon(f64);
impl Sale for OneDollarOffCoupon {
    fn amount(&self) -> f64 {
        self.0 - 1.0
    }
}
struct TenPercentOfPromotion(f64);
impl Sale for TenPercentOfPromotion {
    fn amount(&self) -> f64 {
        self.0 * 0.9
    }
}

fn calculate_revenue(sales: &Vec<Box<dyn Sale>>) -> f64 {
    sales.iter().map(|sale| sale.amount()).sum()
}

fn main() {
    let price = 2.0;
    let regular = Box::new(FullSale(price));
    let coupon = Box::new(OneDollarOffCoupon(price));
    let promot = Box::new(TenPercentOfPromotion(price));

    let sales: Vec<Box<dyn Sale>> = vec![regular, coupon, promot];
    let revenue = calculate_revenue(&sales);
    println!("total revenue = {}", revenue);
}
