use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, f64>,
    pub asks: BTreeMap<OrderedFloat<f64>, f64>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn update_bid(&mut self, price: f64, size: f64) {
        self.bids.insert(OrderedFloat(price), size); // ✅ wrap with OrderedFloat
    }

    pub fn update_ask(&mut self, price: f64, size: f64) {
        self.asks.insert(OrderedFloat(price), size); // ✅ wrap with OrderedFloat
    }
}