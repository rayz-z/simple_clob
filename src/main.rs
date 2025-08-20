use clob::order_generator::OrderGenerator;
use clob::OrderBook;

fn main() {
    println!("This is a simple clob");

    let mut ord_book = OrderBook::build();

    // let ord_gen = OrderGenerator::build(0.5, 0.25);


    if let Some(ord_gen) = OrderGenerator::build(0.5, 0.5) {
        for _ in 0..20 {
            let (buy_sell, price) = ord_gen.gen_order(10.0);
            if buy_sell{
                ord_book.buy(true, price, 1);
            } else {
                ord_book.sell(false, price, 1);
            }
        }
    } else {
        println!("Failed to build OrderGenerator!");
    }

    ord_book.display();

    // let mut ord_book = OrderBook::build();

    // ord_book.buy(true, 10);
    // ord_book.buy(true, 12);
    // ord_book.buy(true, 13);
    // ord_book.buy(true, 14);
    // ord_book.buy(true, 19);

    // ord_book.sell(false, 20);
    // ord_book.sell(false, 22);
    // ord_book.sell(false, 23);
    // ord_book.sell(false, 25);
    // ord_book.sell(false, 28);
    // ord_book.sell(false, 29);

    // ord_book.display();
}
