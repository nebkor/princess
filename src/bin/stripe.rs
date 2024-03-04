use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_slash).post(post_slash))
        .into_make_service();
    let addr = SocketAddr::from(([127, 0, 0, 1], 4001));
    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

pub async fn get_slash() -> impl IntoResponse {
    let html: Html<&str> =
        "<p>This is Stripe.</p><p><form action='/' enctype='application/x-www-form-urlencoded' method='post'>
    <input type='submit' value='back to princess'></form></p>"
            .into();
    html.into_response()
}

pub async fn post_slash() -> impl IntoResponse {
    Redirect::to("http://localhost:4000/success").into_response()
}
