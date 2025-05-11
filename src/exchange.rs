use crate::momentum::Action;

#[derive(Debug)]
pub struct Trade {
    pub action: Action,
    pub price: f64,
    pub quantity: f64,
}

#[derive(Debug)]
pub struct Position {
    pub quantity: f64,
    pub avg_cost: f64,
    pub realized_pnl: f64,
}

impl Position {
    pub fn new() -> Self {
        Self {
            quantity: 0.0,
            avg_cost: 0.0,
            realized_pnl: 0.0,
        }
    }

    pub fn execute(&mut self, action: Action, price: f64, quantity: f64) -> Option<Trade> {
        match action {
            Action::Buy => {
                self.avg_cost = if self.quantity == 0.0 {
                    price
                } else {
                    (self.avg_cost * self.quantity + price * quantity) / (self.quantity + quantity)
                };
                self.quantity += quantity;
                Some(Trade { action, price, quantity })
            }
            Action::Sell => {
                if self.quantity >= quantity {
                    self.quantity -= quantity;
                    let pnl = (price - self.avg_cost) * quantity;
                    self.realized_pnl += pnl;
                    Some(Trade { action, price, quantity })
                } else {
                    // Not enough to sell
                    None
                }
            }
            Action::Hold => None,
        }
    }

    pub fn unrealized_pnl(&self, current_price: f64) -> f64 {
        (current_price - self.avg_cost) * self.quantity
    }
}