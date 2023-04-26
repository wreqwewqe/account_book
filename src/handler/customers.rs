use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use axum::{Json, TypedHeader};
use axum::response::IntoResponse;
use axum::{extract::{State}};
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use serde_json::json;
use diesel::prelude::*;
use crate::config::AppError;
use crate::Pool;
use crate::methods::{get_connection, my_decode};
use crate::schema::customers::{self, parent_uuid};
use crate::models::customers::{ CreateCustomer, Customer};
pub async fn create_customer(State(pool): State<Pool>,Json(info):Json<CreateCustomer>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&pool).await?;
    diesel::insert_into(customers::table)
        .values(info)
        .execute(&mut conn)
        .await
        .map_err(|e| AppError::err(500,e.to_string()))?;
    Ok(Json(json!({
        "code":200,
        "msg":"添加客户成功"
    })))
}

pub async fn customer_list(TypedHeader(auth):TypedHeader<Authorization<Bearer>>,State(pool): State<Pool>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&pool).await?;
    let mut query=customers::table.into_boxed();
    let claims=my_decode(auth)?;
    query=query.filter(parent_uuid.eq(claims.uuid));
    let lists=query
                        .load::<Customer>(&mut conn)
                        .await
                        .map_err(|e| AppError::err(500,e.to_string()))?;
    
    Ok(Json(json!({
        "code":200,
        "msg":"请求成功",
        "data":{
            "lists":lists
        }
    })))

}