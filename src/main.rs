use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SIGNUP_KEY: String = format!("meow-{}", rand::random::<u128>());
}

#[tokio::main]
async fn main() {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::hours(2)));

    let app = Router::new()
        .route("/signup", get(get_signup).post(post_signup))
        .route("/payment_success/:receipt", get(payment_success))
        .layer(session_layer)
        .into_make_service();
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

pub async fn get_signup() -> impl IntoResponse {
    let html: Html<&str> =
        "<p><form action='/signup' enctype='application/x-www-form-urlencoded' method='post'>
    <input type='submit' value='Signup'></form></p>"
            .into();
    html.into_response()
}

pub async fn post_signup(session: Session) -> impl IntoResponse {
    let user = TestData {
        name: "yo yo".to_string(),
        ..Default::default()
    };

    session.insert(&SIGNUP_KEY, user).await.unwrap();
    session.save().await.unwrap();

    Redirect::to("http://localhost:4001").into_response()
}

pub async fn payment_success(session: Session) -> impl IntoResponse {
    session.load().await.unwrap();
    dbg!("loaded the session");
    let user = if let Some(user) = session.get::<TestData>(&SIGNUP_KEY).await.unwrap_or(None) {
        dbg!("loaded test data");
        user
    } else {
        dbg!("could not load data from session");
        TestData::default()
    };
    let html: Html<_> = format!("<p>Check out this test data:</p><pre>{user:?}</pre>").into();
    html.into_response()
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
struct TestData {
    name: String,
    login: String,
    email: String,
    password: String,
    thingy: bool,
}
