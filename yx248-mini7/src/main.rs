use axum::{routing::get, Router};
use yx248_mini4::{update_data, price_filter};

//Root Route for Change Machine
async fn root() -> &'static str {
    "
    yx248-mini4

    **Primary Route:**
    /pricefilter/low_price_bound/high_price_bound
    "
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/updatedata", get(update_data))
        .route("/pricefilter/:low/:high", get(price_filter));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

