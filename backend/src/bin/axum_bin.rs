use axum::{
    extract::State, routing::{get, post}, Json, Router
};
use backend::{OrderBook, order_generator::OrderGenerator};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateOrderResponse {
    pub status: String,
    pub order_id: u128,
}

#[derive(Serialize, Deserialize)]
pub struct CreateOrder {
    pub buy_order: bool,
    pub price: u64,
    pub quantity: u64,
}

#[tokio::main]
async fn main() {
    let ord_book = Arc::new(RwLock::new(build_order_book()));

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
        .route("/orders", post(post_orders))
        .with_state(ord_book)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "CLOB API Homepage"
}

async fn clob_stats(State(ord_book): State<Arc<RwLock<OrderBook>>>) -> Json<OrderBook> {
    let ob = ord_book.read().await;
    Json(ob.clone())
}

async fn post_orders(State(ord_book): State<Arc<RwLock<OrderBook>>>, Json(payload): Json<CreateOrder>) -> Json<CreateOrderResponse> {
    let mut ob = ord_book.write().await;
    let ob = ob.buy(payload.buy_order, payload.price, payload.quantity as u128);

    Json(CreateOrderResponse { status: "ok".to_string(), order_id: ob.unwrap() })
}

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
