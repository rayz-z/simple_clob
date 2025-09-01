use std::fmt::Error;
use std::time::SystemTime;
pub mod order_generator;
// pub mod order_match;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

type Price = u64;

// simulate order flow
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    buy_order: bool, // refactor as enum
    price: Price,
    quantity: u128,
    id: u128, // change to str in future
    time_created: SystemTime,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    price: Price,
    quantity: u128,
    time: SystemTime,
}

// one asset
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    total_orders: u128,                       // historic amount
    buy_orders: BTreeMap<Price, Vec<Order>>,  // refactor into Vec<Order>
    sell_orders: BTreeMap<Price, Vec<Order>>, // "        " f64 doesn't implement eq
    transactions: Vec<Transaction>,
}

impl Order {
    pub fn new() -> Self {
        Order {
            buy_order: true,
            price: 0,
            quantity: 0,
            id: 0,
            time_created: SystemTime::now(),
        }
    }
}

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

    pub fn buy(self: &mut Self, buy: bool, price: Price, quantity: u128) -> Result<u128, ()> {
        if quantity == 0 {
            println!("quantity can't be 0");
            return Err(())
        }

        let id = self.total_orders;
        let id2 = id.clone();
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
            return Err(())
        }
        // resolve
        self.resolve();
        return Ok(id2)
    }

    pub fn sell(self: &mut Self, buy: bool, price: Price, quantity: u128) -> Result<u128, ()> {
        if quantity == 0 {
            println!("quantity can't be 0");
            return Err(())
        }

        let id = self.total_orders;
        let id2 = id.clone();
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
            return Err(())
        }
        // resolve
        self.resolve();
        return Ok(id2) // if order resolves this id is still returned
    }

    pub fn market_buy(&self, quantity: u128) {}

    pub fn market_sell(&self,) {}

    pub fn cancel(&mut self, id: u128) -> Result<Order, Error> {
        let mut ord = Order::new();
        let mut remove_sell = false;
        let mut remove_buy = false;

        for (price, orders) in self.buy_orders.iter_mut() {
            let mut index = 0;
            if let Some(_) = orders.iter().find(|b| b.id == id) {
                // remove from buy_orders
                let order = orders.remove(index);

                if orders.is_empty() {
                    remove_buy = true;
                }

                ord = order;
            }
            index += 1;
        }
        for (price, orders) in self.sell_orders.iter_mut() {
            let mut index = 0;
            if let Some(_) = orders.iter().find(|b| b.id == id) {
                // remove from sell_orders
                let order = orders.remove(index);

                if orders.is_empty() {
                    remove_sell = true;
                }

                ord = order;
            }
            index += 1;
        }

        if ord.quantity != 0 {
            if remove_buy {
                self.buy_orders.remove(&ord.price);
            } else if remove_sell {
                self.sell_orders.remove(&ord.price);
            }
            return Ok(ord);
        }

        Err(Error)
    }

    pub fn resolve(self: &mut Self) {
        // need to buy called in buy/sell so the trades get resolved correctly by time they come in
        // Keep resolving orders while there are matching prices
        loop {
            if self.buy_orders.is_empty() || self.sell_orders.is_empty() {
                break;
            }

            let buy_price = if let Some((price, _)) = self.buy_orders.last_key_value() {
                *price
            } else {
                break;
            };

            let sell_price = if let Some((price, _)) = self.sell_orders.first_key_value() {
                *price
            } else {
                break;
            };

            if buy_price < sell_price {
                break;
            }

            let mut should_remove_buy_price = false;
            let mut should_remove_sell_price = false;
            let mut match_quantity = 0;

            // First, get the order details without mutable borrows
            if let Some(buy_orders) = self.buy_orders.get(&buy_price) {
                if let Some(sell_orders) = self.sell_orders.get(&sell_price) {
                    if !buy_orders.is_empty() && !sell_orders.is_empty() {
                        let buy_qty = buy_orders[0].quantity;
                        let sell_qty = sell_orders[0].quantity;
                        match_quantity = std::cmp::min(buy_qty, sell_qty);
                    }
                }
            }

            if match_quantity > 0 {
                // Now process the orders with separate mutable borrows
                if let Some(buy_orders) = self.buy_orders.get_mut(&buy_price) {
                    if let Some(sell_orders) = self.sell_orders.get_mut(&sell_price) {
                        if !buy_orders.is_empty() && !sell_orders.is_empty() {
                            // Update buy order
                            let buy_order = &mut buy_orders[0];
                            buy_order.quantity -= match_quantity;
                            if buy_order.quantity == 0 {
                                buy_orders.remove(0);
                                should_remove_buy_price = buy_orders.is_empty();
                            }

                            // Update sell order
                            let sell_order = &mut sell_orders[0];
                            sell_order.quantity -= match_quantity;
                            if sell_order.quantity == 0 {
                                sell_orders.remove(0);
                                should_remove_sell_price = sell_orders.is_empty();
                            }

                            // Create transaction
                            let transaction = Transaction {
                                price: sell_price, // Use sell price as the match price --> aggro sell uses buy_price?
                                quantity: match_quantity,
                                time: SystemTime::now(),
                            };
                            self.transactions.push(transaction);
                        }
                    }
                }

                // Remove empty price levels
                if should_remove_buy_price {
                    self.buy_orders.remove(&buy_price);
                }
                if should_remove_sell_price {
                    self.sell_orders.remove(&sell_price);
                }
            } else {
                // No more matches possible
                break;
            }
        }
    }

    pub fn display(&self) {
        println!("Order Book Stats");
        println!("-------------------");
        println!("bids");
        for (_, order) in self.buy_orders.iter() {
            for ord in order {
                println!("Bid: price: {}, quantity: {}", ord.price, ord.quantity);
            }
        }
        println!("-------------------");
        println!("asks");
        for (_, order) in self.sell_orders.iter() {
            for ord in order {
                println!("Ask: price: {}, quantity: {}", ord.price, ord.quantity);
            }
        }
        println!("-------------------");
        println!("transactions");
        for transaction in self.transactions.iter() {
            println!(
                "Transaction: price: {}, quantity: {}, time: {:?}",
                transaction.price, transaction.quantity, transaction.time
            );
        }
    }

    pub fn display_depth_chart(&self) {}

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

    pub fn get_tot_orders(&self) -> &u128 {
        &self.total_orders
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
        assert_eq!(a.total_orders, 1);
    }

    #[test]
    fn test_sell() {
        let mut a = OrderBook::build();
        a.sell(false, 2, 1);

        assert_eq!(a.sell_orders.len(), 1);
        assert_eq!(a.total_orders, 1);
    }

    #[test]
    fn test_cancel() {
        let mut a = OrderBook::build();
        a.buy(true, 2, 1);

        let b = a.cancel(0);
        a.display();
        println!("{}", a.buy_orders.len());

        assert_eq!(a.buy_orders.len(), 0);
    }

    #[test]
    fn test_order_matching_buy_aggressive() {
        let mut a = OrderBook::build();
        // Place a sell order first
        a.sell(false, 100, 10);
        // Place a buy order that should match
        a.buy(true, 100, 5);

        // Should have 1 transaction
        assert_eq!(a.transactions.len(), 1);
        // Sell order should still exist with remaining quantity
        assert_eq!(a.sell_orders.len(), 1);
        // Buy order should be fully filled and removed
        assert_eq!(a.buy_orders.len(), 0);
    }

    #[test]
    fn test_order_matching_sell_aggressive() {
        let mut a = OrderBook::build();
        // Place a buy order first
        a.buy(true, 100, 10);
        // Place a sell order that should match
        a.sell(false, 100, 5);

        // Should have 1 transaction
        assert_eq!(a.transactions.len(), 1);
        // Buy order should still exist with remaining quantity
        assert_eq!(a.buy_orders.len(), 1);
        // Sell order should be fully filled and removed
        assert_eq!(a.sell_orders.len(), 0);
    }

    #[test]
    fn test_exact_order_match() {
        let mut a = OrderBook::build();
        // Place a sell order
        a.sell(false, 100, 10);
        // Place a buy order with exact same quantity
        a.buy(true, 100, 10);

        // Should have 1 transaction
        assert_eq!(a.transactions.len(), 1);
        // Both orders should be fully filled and removed
        assert_eq!(a.buy_orders.len(), 0);
        assert_eq!(a.sell_orders.len(), 0);
    }

    #[test]
    fn test_no_matching_orders() {
        let mut a = OrderBook::build();
        // Place a sell order at higher price
        a.sell(false, 100, 10);
        // Place a buy order at lower price (no match)
        a.buy(true, 90, 5);

        // Should have no transactions
        assert_eq!(a.transactions.len(), 0);
        // Both orders should remain
        assert_eq!(a.buy_orders.len(), 1);
        assert_eq!(a.sell_orders.len(), 1);
    }

    #[test]
    fn test_resolve_function() {
        let mut a = OrderBook::build();
        // Place multiple orders that can match
        a.sell(false, 100, 10);
        a.sell(false, 95, 5);
        a.buy(true, 100, 8);
        a.buy(true, 98, 7);

        // Manually resolve all possible matches
        a.resolve();

        // Should have multiple transactions
        assert!(a.transactions.len() > 0);
        // Orders should be properly matched and quantities updated
        assert!(a.buy_orders.len() <= 2);
        assert!(a.sell_orders.len() <= 2);
    }

    #[test]
    fn test_zero_quantity_rejection() {
        let mut a = OrderBook::build();
        // Try to place orders with zero quantity
        a.buy(true, 100, 0);
        a.sell(false, 100, 0);

        // Should have no orders
        assert_eq!(a.buy_orders.len(), 0);
        assert_eq!(a.sell_orders.len(), 0);
        assert_eq!(a.total_orders, 0);
    }

    #[test]
    fn test_order_priority() {
        let mut a = OrderBook::build();
        // Place multiple sell orders at same price
        a.sell(false, 100, 5);
        a.sell(false, 100, 3);
        a.sell(false, 100, 7);

        // Place a buy order that should match the first one
        a.buy(true, 100, 4);

        // Should have 1 transaction
        assert_eq!(a.transactions.len(), 1);
        // First sell order should be partially filled
        assert_eq!(a.sell_orders.len(), 1);
        // Buy order should be fully filled and removed
        assert_eq!(a.buy_orders.len(), 0);
    }

    #[test]
    fn test_transaction_details() {
        let mut a = OrderBook::build();
        a.sell(false, 100, 10);
        a.buy(true, 100, 5);

        // Check transaction details
        assert_eq!(a.transactions.len(), 1);
        let transaction = &a.transactions[0];
        assert_eq!(transaction.price, 100);
        assert_eq!(transaction.quantity, 5);
        assert!(transaction.time > SystemTime::UNIX_EPOCH);
    }
}
