use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use diesel_async::RunQueryDsl;
use serde_json::json;
use diesel::prelude::*;
use crate::config::AppError;
use crate::Pool;
use crate::methods::{get_connection, now};
use crate::models::orders::{CreateOrder, UpdateOrder, Order, QueryOrder};
use crate::schema::orders::{self,id, amount,status};

pub async fn create(State(pool):State<Pool>,Json(mut info):Json<CreateOrder>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&pool).await?;
    info.create_at=Some(now());
    info.status=Some(true);
    let row=diesel::insert_into(orders::table)
                     .values(info)
                     .execute(&mut conn)
                     .await
                     .map_err(|e| AppError::err(500,e.to_string()))?;
    Ok(Json(json!({
        "code":200,
        "msg":"添加成功"
    })))
}


pub async fn update(State(pool):State<Pool>,Json(info):Json<UpdateOrder>)->Result<impl IntoResponse,AppError>{
    println!("我进来了");
    let mut conn=get_connection(&pool).await?;
    println!("info:{:?}",info);
    diesel::update(orders::table.filter(id.eq(info.id)))
        .set((amount.eq(info.amount),status.eq(info.status)))
        .execute(&mut conn)
        .await
        .map_err(|e| AppError::err(500,e.to_string()))?;

    Ok(Json(json!({
        "code":200,
        "msg":"更新成功"
    })))
}

pub async fn list(State(pool):State<Pool>,Json(info):Json<QueryOrder>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&pool).await?;
    println!("我建立好了连接");
    let lists=orders::table
        .offset((info.pagenum-1)*info.pagesize)
        .limit(info.pagesize)
        .load::<Order>(&mut conn)
        .await
        .map_err(|e| AppError::err(500,e.to_string()))?;
    println!("第一次查询完成");
    let total=orders::table
        .load::<Order>(&mut conn)
        .await
        .map_err(|e| AppError::err(500,e.to_string()))?;
    println!("第二次查询完成");
    Ok(Json(json!({
        "code":200,
        "msg":"请求成功",
        "data":{
            "list":lists,
            "total":total.len()
        }
    })))
}