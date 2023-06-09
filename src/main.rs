use axum::{
    extract::{ State, Path, DefaultBodyLimit,ws::{ WebSocketUpgrade, WebSocket,}},
    response::{Json, IntoResponse},
    routing::{get, post,delete, Route},
    Router,
};
use tower_http::services::ServeDir;
use config::{Share, AppError};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use axum::response::Response;
use serde_json::json;
use tokio::task::JoinHandle;
use tower_http::{limit::RequestBodyLimitLayer, cors::Any};
use std::{net::SocketAddr, time::Duration, collections::HashMap, sync::{Arc, Mutex}};

mod methods;
use tokio::sync::broadcast;
mod schema;
mod models;
mod handler;
mod middleware;
mod config;
use tower_http::cors::CorsLayer;
use axum_server::tls_rustls::RustlsConfig;
use std::{ path::PathBuf};
type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
// type Pool=Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[derive(Clone)]
pub struct AppState{
    pool:Pool,
    tx:broadcast::Sender<String>
}
#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("env文件解析失败");

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let (sender,receiver)=broadcast::channel::<String>(200);
    // set up connection pool
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();
    let app_state=AppState{
        pool,
        tx:sender
    };
    // let mut share=Arc::new(Share::new(pool));
    // build our application with some routes

    let auth_routes=Router::new()     
        .route("/api/users/info",post(handler::users::current_user_info))
        .route("/api/users/update",post(handler::users::update_user))
        .route("/api/users/upload",post(handler::users::upload))
        .route("/api/:filename",get(handler::users::get_avatar))
        .route("/api/customer/create",post(handler::customers::create_customer))
        .route("/api/customer/list",post(handler::customers::customer_list))
        .route("/api/customer/update",post(handler::customers::update_customer))
        .route("/api/customer/delete",post(handler::customers::delete_customer))
        .route("/api/orders/create",post(handler::orders::create))
        .route("/api/orders/update",post(handler::orders::update))
        .route("/api/orders/list",post(handler::orders::list))
        .route("/api/orders/delete",post(handler::orders::delete))
        // .route("/test",post(handler::orders::test))
        .layer(axum::middleware::from_fn(middleware::auth));
    // let ssl_config = RustlsConfig::from_pem_file(
    //     // PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //     //     .join("self_signed_certs")
    //     //     .join("cert.pem"),
    //     // PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    //     //     .join("self_signed_certs")
    //     //     .join("key.pem"),
    //     PathBuf::from("./self_signed_certs/crt.pem"),
    //     PathBuf::from("./self_signed_certs/key.pem")
    // )
    // .await
    // .unwrap();
    // println!("ssl_config:{:?}",ssl_config);

    let noauth_routes=Router::new()
        .route("/websocket",get(handler::websocket::websocket_handler))
        .route("/api/users/create",post(handler::users::create_user))
        .route("/api/login",post(handler::users::login))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new( 250 * 1024 * 1024));  

    let app = Router::new()
        .merge(auth_routes)
        .merge(noauth_routes)   
        .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any))
        .with_state(app_state);
    
        // .layer(axum::middleware::from_fn(middleware::my_middleware1))
        // .layer(axum::middleware::from_fn(middleware::my_middleware2));
    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 5001));
    //ssl
    // axum_server::bind_rustls(addr,ssl_config)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap(); 
    
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap()
}


