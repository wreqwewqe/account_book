use diesel::prelude::*;

use crate::schema::customers;

#[derive(Debug,Queryable,serde::Deserialize,serde::Serialize)]
pub struct Customer{
    pub id:i32,
    pub parent_uuid:String,
    pub customer_name:String,
    pub total_debts:i32,
}

#[derive(Debug,serde::Deserialize,Insertable)]
#[diesel(table_name =customers)]
pub struct CreateCustomer{
    pub parent_uuid:String,
    pub customer_name:String
}

