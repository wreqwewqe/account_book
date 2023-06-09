use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use diesel_async::RunQueryDsl;
use serde_json::json;
use diesel::prelude::*;
use crate::config::AppError;
use crate::Pool;
use crate::methods::{get_connection, now};
use crate::models::orders::{CreateOrder, UpdateOrder, Order, QueryOrder, CountOrder, DeleteOrder};
use crate::schema::customers::{self,customer_name};
use crate::schema::orders::{self,id, amount,status,customer_id,create_at, remark};
use crate::AppState;
pub async fn create(State(app_state):State<AppState>,Json(mut info):Json<CreateOrder>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&app_state.pool).await?;
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


pub async fn update(State(app_state):State<AppState>,Json(info):Json<UpdateOrder>)->Result<impl IntoResponse,AppError>{
    println!("我进来了");
    let mut conn=get_connection(&app_state.pool).await?;
    println!("info:{:?}",info);
    diesel::update(orders::table.filter(id.eq(info.id)))
        .set((amount.eq(info.amount),status.eq(info.status),remark.eq(info.remark)))
        .execute(&mut conn)
        .await
        .map_err(|e| AppError::err(500,e.to_string()))?;

    Ok(Json(json!({
        "code":200,
        "msg":"更新成功"
    })))
}

joinable!(orders -> customers(customer_id));
allow_tables_to_appear_in_same_query!(orders,customers);
pub async fn list(State(app_state):State<AppState>,Json(info):Json<QueryOrder>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&app_state.pool).await?;
    println!("我建立好了连接");
    let  mut query1=orders::table.into_boxed()
                    .left_join(customers::table)
                    .select((id,customer_id,customer_name.nullable(),amount,status,create_at,remark));
    let  mut query2=orders::table.into_boxed()
                    .left_join(customers::table)
                    .select((id,customer_id,customer_name.nullable(),amount,status,create_at,remark));
    if let Some(v)=info.customer_name{
        query1=query1.filter(customer_name.like(v.clone()+"%"));
        query2=query2.filter(customer_name.like(v+"%"));
    }
    if let Some(v)=info.status{
        query1=query1.filter(status.eq(v));
        query2=query2.filter(status.eq(v));
    }
    println!("info.status:{:?}",info.status);
    let lists=query1
        .offset((info.pagenum-1)*info.pagesize)
        .limit(info.pagesize)
        .load::<Order>(&mut conn)
        .await
        .map_err(|e| AppError::err(500,e.to_string()))?;
    println!("第一次查询完成");
    let total=query2
        .load::<Order>(&mut conn)
        .await
        .map_err(|e| AppError::err(500,e.to_string()))?;
    println!("第二次查询完成");
    Ok(Json(json!({
        "code":200,
        "msg":"请求成功",
        "data":{
            "lists":lists,
            "total":total.len()
        }
    })))
}

pub async fn delete(State(app_state):State<AppState>,Json(info):Json<DeleteOrder>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&app_state.pool).await?;
    let r=diesel::delete(orders::table.filter(id.eq(info.id)))
        .execute(&mut conn)
        .await
        .map_err(|e| AppError::err(500,e.to_string()))?;
    Ok(Json(json!({
        "code":200,
        "msg":"删除成功"
    })))
}


