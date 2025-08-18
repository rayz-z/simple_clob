use clob::order_generator::OrderGenerator;
use clob::OrderBook;

fn main() {
    println!("This is a simple clob");

    let mut ord_book = OrderBook::build();

    if let Some(ord_gen) = OrderGenerator::build(0.5, 0.025) {
        for _ in 0..10 {
            let (buy_sell, price) = ord_gen.gen_order(10.0);
            if buy_sell{
                ord_book.buy(true, price);
            } else {
                ord_book.sell(false, price);
            }
        }
    } else {
        println!("Failed to build OrderGenerator!");
    }

    ord_book.display();
}
