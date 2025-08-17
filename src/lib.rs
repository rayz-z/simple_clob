use std::fmt::Error;
use std::time::SystemTime;
// pub mod order_generator;
// pub mod order_match;
use std::collections::BTreeMap;

type Price = u64;

// simulate order flow
#[derive(PartialEq, Eq, Debug)]
pub struct Order {
    buy_order: bool,
    price: Price,
    id: u128, // change to str in future
    time_created: SystemTime,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Transaction {
    price: Price,
    time: SystemTime,
}

// pub trait Price {
//     fn display(&self) {}
// }

// one asset
#[derive(PartialEq, Eq, Debug)]
pub struct OrderBook {
    total_orders: u128,                 // historic amount
    buy_orders: BTreeMap<Price, Order>,  // refactor into Vec<Order>
    sell_orders: BTreeMap<Price, Order>, // "        " f64 doesn't implement eq
    transactions: Vec<Transaction>,
}

// impl Price for Order {
//     fn display(&self) {
//     }
// }

// clear orders
impl OrderBook {
    fn build() -> Self {
        OrderBook {
            total_orders: 0,
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
            transactions: Vec::new(),
        }
    }

    pub fn buy(self: &mut Self, buy: bool, price: Price) {
        let id = self.total_orders;
        if buy {
            self.buy_orders.insert(
                price,
                Order {
                    buy_order: buy,
                    price,
                    id,
                    time_created: SystemTime::now(),
                },
            );
            self.total_orders += 1;
        } else {
            println!("not a buy order");
        }
    }

    pub fn sell(self: &mut Self, buy: bool, price: Price) {
        let id = self.total_orders;
        if !buy {
            self.sell_orders.insert(
                price,
                Order {
                    buy_order: buy,
                    price,
                    id,
                    time_created: SystemTime::now(),
                },
            );
            self.total_orders += 1;
        } else {
            println!("not a sell order");
        }
    }

    pub fn cancel(&mut self, id: u128) -> Result<Order, Error> {
        if let Some((price, _)) = self.buy_orders.iter().find(|(_, b)| b.id == id) {
            return Ok(self.buy_orders.remove(&price.clone()).unwrap());
        }
        if let Some((price, _)) = self.sell_orders.iter().find(|(_, b)| b.id == id) {
            return Ok(self.sell_orders.remove(&price.clone()).unwrap());
        }
        Err(Error)
    }

    pub fn resolve(self: &mut Self) {
        if self.buy_orders.last_entry().unwrap().key()
            >= self.sell_orders.first_entry().unwrap().key()
        {
            self.buy_orders.pop_last();
            let (trans, _) = self.sell_orders.pop_first().unwrap();
            self.transactions.push(Transaction {
                price: trans,
                time: SystemTime::now(),
            });
        }
    }

    pub fn display(&self) {
        println!("Order Book Stats");
        println!();
        println!("bids");
        for (bid, order) in self.buy_orders.iter() {
            println!("{}", bid);
        }
        println!();
        println!("asks");
        for (ask, order) in self.sell_orders.iter() {
            println!("{}", ask);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_order_book() {
        let a = OrderBook::build();
        let b = OrderBook {
            total_orders: 0,
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
            transactions: Vec::new(),
        };

        assert_eq!(a, b);
    }

    #[test]
    fn test_buy() {
        let mut a = OrderBook::build();
        a.buy(true, 2);

        assert_eq!(a.buy_orders.len(), 1);
    }

    #[test]
    fn test_sell() {
        let mut a = OrderBook::build();
        a.sell(false, 2);

        assert_eq!(a.sell_orders.len(), 1);
    }

    #[test]
    fn test_cancel() {
        let mut a = OrderBook::build();
        a.buy(true, 2);

        let b = a.cancel(0);

        assert_eq!(a.buy_orders.len(), 0);
    }

    #[test]
    fn test_resolve() {
        let mut a = OrderBook::build();
        a.sell(false, 2);
        a.buy(true, 2);

        a.resolve();

        assert_eq!(a.transactions.len(), 1);
        assert_eq!(a.buy_orders.len(), 0);
        assert_eq!(a.sell_orders.len(), 0);
    }

}