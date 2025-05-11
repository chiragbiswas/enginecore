use rand::Rng;

pub struct Market {
    pub base_price: f64,
}

impl Market {
    pub fn new(base_price: f64) -> Self {
        Self { base_price }
    }

    pub fn next_tick(&mut self) -> (f64, f64) {
        let mut rng = rand::thread_rng();

        //simulate small price movements
        let spread = rng.gen_range(0.2..0.5);
        let mid_price_change = rng.gen_range(-0.5..0.5);
        self.base_price += mid_price_change;

        let bid = (self.base_price - spread / 2.0).max(1.0);
        let ask = (self.base_price + spread / 2.0).max(bid + 0.01); // ensure ask > bid

        (bid, ask)
    }
}


