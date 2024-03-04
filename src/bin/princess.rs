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
        .route("/", get(get_slash).post(post_slash))
        .route("/success", get(success))
        .layer(session_layer)
        .into_make_service();
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

pub async fn get_slash() -> impl IntoResponse {
    let html: Html<&str> =
        "<p>Welcome to Princess</p><p><form action='/' enctype='application/x-www-form-urlencoded' method='post'>
    <input type='submit' value='go to stripe'></form></p>"
            .into();
    html.into_response()
}

pub async fn post_slash(session: Session) -> impl IntoResponse {
    let user = TestData {
        name: "yo yo".to_string(),
        ..Default::default()
    };

    session.insert(&SIGNUP_KEY, user).await.unwrap();
    session.save().await.unwrap();

    let mut args = std::env::args();
    args.next();
    let stripe = args.next().unwrap_or("http://localhost:4001/".into());

    Redirect::to(&stripe).into_response()
}

pub async fn success(session: Session) -> impl IntoResponse {
    session.load().await.unwrap();
    let td = if let Some(td) = session.get::<TestData>(&SIGNUP_KEY).await.unwrap_or(None) {
        td
    } else {
        TestData::default()
    };
    let td = if td == TestData::default() {
        "Default TestData, session fetch failed.".to_string()
    } else {
        format!("Successful fetch from session: {td:?}")
    };
    let html: Html<_> = format!("<p>Check out this test data:</p><pre>{td}</pre>").into();
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
