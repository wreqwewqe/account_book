use axum::{TypedHeader, headers::{Authorization, authorization::Bearer}, http::Request, middleware::Next, response::Response};

pub async fn auth<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    // do something with `request`...
    let response = next.run(request).await;

    // do something with `response`...
    // println!("中间件2执行完毕");
    response
}