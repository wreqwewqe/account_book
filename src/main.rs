use axum::{
    extract::{ State, Path, DefaultBodyLimit},
    response::{Json, IntoResponse},
    routing::{get, post,delete, Route},
    Router,
};
use config::{Share, AppError};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use axum::response::Response;
use serde_json::json;
use tokio::task::JoinHandle;
use tower_http::limit::RequestBodyLimitLayer;
use std::{net::SocketAddr, time::Duration, collections::HashMap, sync::{Arc, Mutex}};
mod methods;

mod schema;
mod models;
mod handler;
mod middleware;
mod config;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
// type Pool=Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("env文件解析失败");

    let db_url = std::env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();
    // let mut share=Arc::new(Share::new(pool));
    // build our application with some routes

    let auth_routes=Router::new()
        .route("/customer/create",post(handler::customers::create_customer))
        .route("/customer/list",post(handler::customers::customer_list))
        .route("/orders/create",post(handler::orders::create))
        .route("/orders/update",post(handler::orders::update))
        .layer(axum::middleware::from_fn(middleware::auth));

    let noauth_routes=Router::new()
        .route("/users/create",post(handler::users::create_user))
        .route("/login",post(handler::users::login))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new( 250 * 1024 * 1024));  

    let app = Router::new()
        .merge(auth_routes)
        .merge(noauth_routes)   
        .with_state(pool);
        // .layer(axum::middleware::from_fn(middleware::my_middleware1))
        // .layer(axum::middleware::from_fn(middleware::my_middleware2));
    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 5001));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap(); 
}


