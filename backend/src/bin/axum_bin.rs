use axum::{
    Json,
    {Router, extract::State, routing::get},
};
use backend::{OrderBook, order_generator::OrderGenerator};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let ord_book = Arc::new(build_order_book());

    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(home))
        .route("/clob-stats", get(clob_stats))
        .with_state(ord_book)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "CLOB API Homepage"
}

async fn clob_stats(State(ord_book): State<Arc<OrderBook>>) -> Json<OrderBook> {
    Json(ord_book.as_ref().clone())
}

async fn post_orders() {}

fn build_order_book() -> OrderBook {
    let mut ord_book = OrderBook::build();

    if let Some(ord_gen) = OrderGenerator::build(0.5, 0.5) {
        for _ in 0..20 {
            let (buy_sell, price) = ord_gen.gen_order(10.0);
            if buy_sell {
                ord_book.buy(true, price, 1);
            } else {
                ord_book.sell(false, price, 1);
            }
        }
    } else {
        println!("Failed to build OrderGenerator!");
    }

    ord_book
}
