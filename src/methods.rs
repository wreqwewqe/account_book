use axum::{extract::State, headers::{authorization::Bearer, Authorization}};
use bb8::{PooledConnection, ManageConnection};
use chrono::prelude::*;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use jsonwebtoken::{decode, DecodingKey, Validation, TokenData};
use serde::Deserialize;
use serde_json::{Deserializer, Value};

use crate::{config::AppError, Pool, handler::users::Claims};

pub fn now()->String{
    Local::now().timestamp().to_string()
}

pub fn my_decode(auth:Authorization<Bearer>)->Result<Claims,AppError>{
    println!("auth:{:?}",auth);
    decode::<Claims>(&auth.token(), &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).map_err(|e| AppError::err(500,e.to_string())).and_then(|e| Ok(e.claims))
}


pub async fn get_connection(pool:&Pool)->Result<PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>,AppError>{
    pool.get().await.map_err(|e| AppError::err(500,e.to_string()))
}