
use diesel::prelude::*;
use crate::schema::orders;
#[derive(Debug,serde::Deserialize,serde::Serialize,Queryable)]
#[diesel(table_name=orders)]
pub struct Order{
    pub id:i32,
    pub customer_id:i32,
    pub customer_name:Option<String>,
    pub amount:i32,
    pub status:bool,
    pub create_at:String,
    pub remark:Option<String>
}

#[derive(Debug,serde::Deserialize,serde::Serialize,Queryable)]
#[diesel(table_name=orders)]
pub struct CountOrder{
    pub id:i32,
    pub customer_id:i32,
    pub amount:i32,
    pub status:bool,
    pub create_at:String,
    pub remark:Option<String>
}

//创建订单
#[derive(Debug,Insertable,serde::Deserialize)]
#[diesel(table_name=orders)]
pub struct CreateOrder{
    pub customer_id:i32,
    pub amount:i32,
    pub status:Option<bool>,
    pub create_at:Option<String>,
    pub remark:String
}


// 更新
#[derive(Debug,serde::Serialize,serde::Deserialize)]
pub struct UpdateOrder{
    pub id:i32,
    pub amount:i32,
    pub status:bool,
    pub remark:Option<String>
}


#[derive(Debug,serde::Deserialize,Clone)]
pub struct QueryOrder{
    pub pagenum:i64,
    pub pagesize:i64,
    pub customer_name:Option<String>,
    pub status:Option<bool>
}