# CLOB (Central Limit Order Book) Implementation

A Rust implementation of a central limit order book that simulates order flow and matching for financial trading systems.

## Features

- **Order Management**: Place, cancel, and track buy/sell orders
- **Automatic Order Matching**: Orders are automatically matched when prices cross
- **Price-Time Priority**: Orders are processed in FIFO order at each price level
- **Transaction Recording**: All matched orders create transaction records
- **Real-time Resolution**: Orders are resolved immediately upon placement
- **Borrow Checker Safe**: Written with Rust's ownership system in mind

## Architecture

### Core Components

- **OrderBook**: Main structure managing buy/sell orders and transactions
- **Order**: Individual order with price, quantity, ID, and timestamp
- **Transaction**: Record of matched orders with price, quantity, and timestamp

### Data Structures

- **Buy Orders**: BTreeMap<Price, Vec<Order>> - sorted by price (highest first)
- **Sell Orders**: BTreeMap<Price, Vec<Order>> - sorted by price (lowest first)
- **Transactions**: Vec<Transaction> - history of all matches

## Usage

### Basic Order Placement

```rust
use clob::{OrderBook, Order};

let mut order_book = OrderBook::build();

// Place a buy order
order_book.buy(true, 100, 10);  // Buy 10 units at price 100

// Place a sell order
order_book.sell(false, 100, 5); // Sell 5 units at price 100
```

### Order Matching

Orders are automatically matched when:

- Buy price >= Sell price
- Both orders have remaining quantity

```rust
let mut order_book = OrderBook::build();

// Place sell order first
order_book.sell(false, 100, 10);

// Place buy order that matches
order_book.buy(true, 100, 5);

// Transaction is automatically created
assert_eq!(order_book.transactions.len(), 1);
```

### Manual Resolution

Use the `resolve()` function to manually process all possible matches:

```rust
let mut order_book = OrderBook::build();

// Place multiple orders
order_book.sell(false, 100, 10);
order_book.sell(false, 95, 5);
order_book.buy(true, 100, 8);
order_book.buy(true, 98, 7);

// Manually resolve all matches
order_book.resolve();
```

### Order Cancellation

```rust
let mut order_book = OrderBook::build();
order_book.buy(true, 100, 10);

// Cancel order by ID
let result = order_book.cancel(0);
assert!(result.is_ok());
```

## API Reference

### OrderBook Methods

- `build()` - Create a new empty order book
- `buy(buy: bool, price: Price, quantity: u128)` - Place a buy order
- `sell(buy: bool, price: Price, quantity: u128)` - Place a sell order
- `cancel(id: u128) -> Result<Order, Error>` - Cancel an order by ID
- `resolve()` - Manually resolve all possible order matches
- `display()` - Print current order book state

### Order Properties

- `buy_order: bool` - True for buy, false for sell
- `price: Price` - Order price (u64)
- `quantity: u128` - Order quantity
- `id: u128` - Unique order identifier
- `time_created: SystemTime` - Order creation timestamp

### Transaction Properties

- `price: Price` - Execution price
- `quantity: u128` - Matched quantity
- `time: SystemTime` - Execution timestamp

## Order Matching Logic

1. **Price Priority**: Orders are matched by price (best price first)
2. **Time Priority**: At the same price, orders are matched FIFO
3. **Quantity Handling**: Orders are partially filled if quantities don't match exactly
4. **Automatic Cleanup**: Fully filled orders are removed, empty price levels are cleaned up

## Testing

Run the comprehensive test suite:

```bash
cargo test
```

Tests cover:

- Basic order placement and cancellation
- Order matching scenarios (aggressive buy/sell)
- Exact quantity matches
- No-match scenarios
- Manual resolution
- Edge cases (zero quantity, order priority)

## Performance Characteristics

- **Order Placement**: O(log n) for price level lookup
- **Order Matching**: O(1) for immediate matches
- **Order Cancellation**: O(n) for order search
- **Memory**: Efficient BTreeMap usage for price-ordered storage

## Safety Features

- **Borrow Checker Compliance**: All code compiles without borrow checker conflicts
- **Error Handling**: Proper Result types for operations that can fail
- **Input Validation**: Rejects orders with zero quantity
- **Memory Safety**: No unsafe code, proper ownership patterns

## Future Enhancements

- [ ] Support for different order types (market, stop-loss)
- [ ] Order book depth visualization
- [ ] Performance metrics and monitoring
- [ ] WebSocket API for real-time updates
- [ ] Database persistence for order history
- [ ] Multi-asset support
