use crate::order_book::OrderBook;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone)]
pub enum Action {
    Buy,
    Sell,
    Hold,
}

pub struct MomentumStrategy {
    pub window: Vec<f64>,
    pub max_len: usize,
    pub threshold: f64,
}

impl MomentumStrategy {
    pub fn new(max_len: usize, threshold: f64) -> Self {
        Self {
            window: Vec::with_capacity(max_len),
            max_len,
            threshold,
        }
    }

    pub fn update(&mut self, latest_bid: f64) -> Action {
        self.window.push(latest_bid);

        //keep only the most recent N values
        if self.window.len() > self.max_len {
            self.window.remove(0);
        }

        if self.window.len() < 2 {
            return Action::Hold;
        }

        let prev = self.window[self.window.len() - 2];
        let curr = self.window[self.window.len() - 1];
        let change = curr - prev;

        if change > self.threshold {
            Action::Buy
        } else if change < -self.threshold {
            Action::Sell
        } else {
            Action::Hold
        }
    }
}