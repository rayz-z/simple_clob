use std::fmt::Error;
use std::time::SystemTime;
pub mod order_generator;
// pub mod order_match;
use std::collections::BTreeMap;

type Price = u64;

// simulate order flow
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Order {
    buy_order: bool, // refactor as enum
    price: Price,
    quantity: u128,
    id: u128, // change to str in future
    time_created: SystemTime,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Transaction {
    price: Price,
    quantity: u128,
    time: SystemTime,
}

// pub trait Price {
//     fn display(&self) {}
// }

// one asset
#[derive(PartialEq, Eq, Debug)]
pub struct OrderBook {
    total_orders: u128,                       // historic amount
    buy_orders: BTreeMap<Price, Vec<Order>>,  // refactor into Vec<Order>
    sell_orders: BTreeMap<Price, Vec<Order>>, // "        " f64 doesn't implement eq
    transactions: Vec<Transaction>,
}

// impl Price for Order {
//     fn display(&self) {
//     }
// }
impl Transaction {
    pub fn new() -> Self {
        Transaction {
            price: 0,
            quantity: 0,
            time: SystemTime::now(),
        }
    }
}

// clear orders
impl OrderBook {
    pub fn build() -> Self {
        OrderBook {
            total_orders: 0,
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
            transactions: Vec::new(),
        }
    }

    pub fn buy(self: &mut Self, buy: bool, price: Price, quantity: u128) {
        if quantity == 0 {
            println!("quantity can't be 0");
            return;
        }

        let id = self.total_orders;
        if buy {
            self.buy_orders
                .entry(price)
                .or_insert_with(Vec::new)
                .push(Order {
                    buy_order: buy,
                    price,
                    quantity,
                    id,
                    time_created: SystemTime::now(),
                });
            self.total_orders += 1;
        } else {
            println!("not a buy order");
            return;
        }

        // while there are still sells <= buy price --> buy and push that to part of the transaction
        // or resolve when quantity hits 0
        let mut trans = Transaction::new();

        // resolve/loop
        self.transactions.push(trans);
    }

    pub fn sell(self: &mut Self, buy: bool, price: Price, quantity: u128) {
        if quantity == 0 {
            println!("quantity can't be 0");
            return;
        }

        let id = self.total_orders;
        if !buy {
            self.sell_orders
                .entry(price)
                .or_insert_with(Vec::new)
                .push(Order {
                    buy_order: buy,
                    price,
                    quantity,
                    id,
                    time_created: SystemTime::now(),
                });
            self.total_orders += 1;
        } else {
            println!("not a sell order");
            return;
        }

        let mut trans = Transaction::new();

       // resolve
        self.transactions.push(trans);
    }

    pub fn cancel(&mut self, id: u128) -> Result<Order, Error> {
        for (_, orders) in self.buy_orders.iter_mut() {
            let mut index = 0;
            if let Some(_) = orders.iter().find(|b| b.id == id) {
                return Ok(orders.remove(index));
            }
            index += 1;
        }
        for (_, orders) in self.sell_orders.iter_mut() {
            let mut index = 0;
            if let Some(_) = orders.iter().find(|b| b.id == id) {
                return Ok(orders.remove(index));
            }
            index += 1;
        }
        Err(Error)
    }

    pub fn resolve(self: &mut Self) {
        // change for quantity
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

        while {
            let buy_order = self.get_mut_buy_order(id).unwrap();
            buy_order.quantity > 0
        } {
            if self.sell_orders.is_empty() {
                break;
            }

            let mut entry = self.sell_orders.first_entry().unwrap();
            let sell_price = *entry.key();
            if price < sell_price {
                break;
            }

            let orders = entry.get_mut();
            if let Some(ord) = orders.first_mut() {
                let buy_order = self.get_mut_buy_order(id).unwrap();

                if ord.quantity > buy_order.quantity {
                    ord.quantity -= buy_order.quantity;

                    trans.quantity += buy_order.quantity;
                    trans.time = SystemTime::now();
                    buy_order.quantity = 0;
                    drop(buy_order);

                    self.cancel(id);
                } else if ord.quantity == buy_order.quantity {
                    trans.quantity += buy_order.quantity;
                    trans.time = SystemTime::now();
                    buy_order.quantity = 0;
                    drop(buy_order);

                    orders.remove(0);
                    self.cancel(id);
                } else {
                    buy_order.quantity -= ord.quantity;

                    trans.quantity += ord.quantity;
                    trans.time = SystemTime::now();

                    orders.remove(0);
                }
            }

            if orders.is_empty() {
                entry.remove();
            }
        }
    }

    pub fn display(&self) {
        println!("Order Book Stats");
        println!("-------------------");
        println!("bids");
        for (bid, order) in self.buy_orders.iter() {
            for ord in order {
                println!("Bid price: {}", ord.price);
            }
        }
        println!("-------------------");
        println!("asks");
        for (ask, order) in self.sell_orders.iter() {
            for ord in order {
                println!("Bid price: {}", ord.price);
            }
        }
    }

    pub fn get_buy_order(&self, id: u128) -> Result<&Order, Error> {
        for (_, orders) in self.buy_orders.iter() {
            if let Some(ord) = orders.iter().find(|b| b.id == id) {
                return Ok(ord);
            }
        }
        Err(Error)
    }

    pub fn get_mut_buy_order(self: &mut Self, id: u128) -> Result<&mut Order, Error> {
        for (_, orders) in self.buy_orders.iter_mut() {
            if let Some(ord) = orders.iter_mut().find(|b| b.id == id) {
                return Ok(ord);
            }
        }
        Err(Error)
    }

    pub fn get_sell_order(&self, id: u128) -> Result<&Order, Error> {
        for (_, orders) in self.sell_orders.iter() {
            if let Some(ord) = orders.iter().find(|b| b.id == id) {
                return Ok(ord);
            }
        }
        Err(Error)
    }

    pub fn get_mut_sell_order(self: &mut Self, id: u128) -> Result<&mut Order, Error> {
        for (_, orders) in self.sell_orders.iter_mut() {
            if let Some(ord) = orders.iter_mut().find(|b| b.id == id) {
                return Ok(ord);
            }
        }
        Err(Error)
    }

    pub fn get_buy_order_quantity(&self, id: u128) -> Result<u128, Error> {
        for (_, orders) in self.buy_orders.iter() {
            if let Some(ord) = orders.iter().find(|b| b.id == id) {
                return Ok(ord.quantity);
            }
        }
        Err(Error)
    }

    pub fn get_sell_order_quantity(&self, id: u128) -> Result<u128, Error> {
        for (_, orders) in self.sell_orders.iter() {
            if let Some(ord) = orders.iter().find(|b| b.id == id) {
                return Ok(ord.quantity);
            }
        }
        Err(Error)
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
        a.buy(true, 2, 1);

        assert_eq!(a.buy_orders.len(), 1);
    }

    #[test]
    fn test_sell() {
        let mut a = OrderBook::build();
        a.sell(false, 2, 1);

        assert_eq!(a.sell_orders.len(), 1);
    }

    #[test]
    fn test_cancel() {
        let mut a = OrderBook::build();
        a.buy(true, 2, 1);

        let b = a.cancel(0);

        assert_eq!(a.buy_orders.len(), 0);
    }

    // #[test]
    // fn test_resolve() {
    //     let mut a = OrderBook::build();
    //     a.sell(false, 2);
    //     a.buy(true, 2);

    //     a.resolve();

    //     assert_eq!(a.transactions.len(), 1);
    //     assert_eq!(a.buy_orders.len(), 0);
    //     assert_eq!(a.sell_orders.len(), 0);
    // }

    // add test for multiple orders
}
