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
    
        if self.window.len() > self.max_len {
            self.window.remove(0);
        }
    
        println!("Current window: {:?}", self.window);
    
        if self.window.len() < self.max_len {
            println!("Holding — window not full yet.");
            return Action::Hold;
        }
    
        let oldest = self.window[0];
        let newest = self.window[self.window.len() - 1];
        let change = newest - oldest;
    
        
        if change > self.threshold {
            println!("→ BUY");
            Action::Buy
        } else if change < -self.threshold {
            println!("→ SELL");
            Action::Sell
        } else {
            println!("→ HOLD");
            Action::Hold
        }
    }
}