use diesel::prelude::*;

use crate::schema::customers;

#[derive(Debug,Queryable,serde::Deserialize,serde::Serialize)]
pub struct Customer{
    pub id:i32,
    pub parent_uuid:String,
    pub customer_name:String,
    pub phone:Option<String>,
    pub total_debts:Option<i32>,
}

#[derive(Debug,serde::Deserialize)]
pub struct QueryCustomer{
    pub pagenum:i64,
    pub pagesize:i64,
    pub customer_name:Option<String>,
   
}

#[derive(Debug,serde::Deserialize,Insertable)]
#[diesel(table_name =customers)]
pub struct CreateCustomer{
    pub parent_uuid:String,
    pub customer_name:String,
    pub phone:Option<String>,
}

#[derive(Debug,serde::Deserialize)]
pub struct UpdateCustomer{
    pub id:i32,
    pub customer_name:String,
    pub phone:String
}


#[derive(Debug,serde::Deserialize)]
pub struct DeleteCustomer{
    pub id:i32,
}