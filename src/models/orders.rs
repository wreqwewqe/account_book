
use diesel::prelude::*;
use crate::schema::orders;
#[derive(Debug,serde::Deserialize,serde::Serialize,Queryable)]
#[diesel(table_name=orders)]
pub struct Order{
    pub id:i32,
    pub customer_id:i32,
    pub amount:i32,
    pub status:bool,
    pub create_at:String
}

//创建订单
#[derive(Debug,Insertable,serde::Deserialize)]
#[diesel(table_name=orders)]
pub struct CreateOrder{
    pub customer_id:i32,
    pub amount:i32,
    pub status:Option<bool>,
    pub create_at:Option<String>
}


// 更新
#[derive(Debug,serde::Serialize,serde::Deserialize)]
pub struct UpdateOrder{
    pub id:i32,
    pub amount:i32,
    pub status:bool,
}


#[derive(Debug,serde::Deserialize)]
pub struct QueryOrder{
    pub pagenum:i64,
    pub pagesize:i64
}