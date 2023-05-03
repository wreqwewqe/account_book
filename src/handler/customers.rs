use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use axum::{Json, TypedHeader};
use axum::response::IntoResponse;
use axum::{extract::{State}};
use diesel::QueryDsl;
use diesel::dsl::count;
use diesel_async::RunQueryDsl;
use serde_json::json;
use diesel::prelude::*;
use crate::config::AppError;
use crate::Pool;
use crate::methods::{get_connection, my_decode};
use crate::schema::customers::{self, parent_uuid, id, customer_name, phone};
use crate::models::customers::{ CreateCustomer, Customer, UpdateCustomer, DeleteCustomer, QueryCustomer};
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

pub async fn update_customer(State(pool):State<Pool>,Json(info):Json<UpdateCustomer>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&pool).await?;
   
    diesel::update(customers::table)
        .filter(id.eq(info.id))
        .set((customer_name.eq(info.customer_name),phone.eq(info.phone)))
        .execute(&mut conn)
        .await
        .map_err(|e| AppError::err(500,e.to_string()))?;
    Ok(Json(json!({
        "code":200,
        "msg":"更新成功"
    })))
}

pub async fn delete_customer(State(pool):State<Pool>,Json(info):Json<DeleteCustomer>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&pool).await?;
    diesel::delete(customers::table.filter(id.eq(info.id)))
        .execute(&mut conn)
        .await
        .map_err(|e| AppError::err(500,e.to_string()))?;
    Ok(Json(json!({
        "code":200,
        "msg":"删除成功"
    })))
}

pub async fn customer_list(TypedHeader(auth):TypedHeader<Authorization<Bearer>>,State(pool):State<Pool>,Json(info):Json<QueryCustomer>)->Result<impl IntoResponse,AppError>{
    println!("我ccccc");
    let mut conn=get_connection(&pool).await?;
    let mut query=customers::table.into_boxed();
    //总数查询
    let mut count_query=customers::table.into_boxed();
    println!("qqqq:");
    let claims=my_decode(auth)?;
    println!("claims:{:?}",claims);
    query=query.filter(parent_uuid.eq(claims.uuid.clone()));
    count_query=count_query.filter(parent_uuid.eq(claims.uuid));
    if let Some(value)=info.customer_name{
        query=query.filter(customer_name.like(value.clone()+"%"));
        count_query=count_query.filter(customer_name.like(value+"%"))
    }

    let lists=query
                        .offset((info.pagenum-1)*info.pagesize)
                        .limit(info.pagesize)
                        .load::<Customer>(&mut conn)
                        .await
                        .map_err(|e| AppError::err(500,e.to_string()))?;
    
    //查询记录总数           
    let count=count_query
            .execute(&mut conn)
            .await
            .map_err(|e| AppError::err(500,e.to_string()))?;
    Ok(Json(json!({
        "code":200,
        "msg":"请求成功",
        "data":{
            "lists":lists,
            "total":count
        }
    })))

}