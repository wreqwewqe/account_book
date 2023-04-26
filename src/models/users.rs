use diesel::prelude::*;

use crate::schema::users;
#[derive(Debug,serde::Deserialize,Queryable)]
pub struct User{
    pub uuid:String,
    pub username:String,
    pub password:String,
    pub create_at:String

}


#[derive(serde::Deserialize,Debug,Insertable)]
#[diesel(table_name =users)]
pub struct CreateUser{
    pub uuid:Option<String>,
    pub username:String,
    pub password:String,
    pub create_at:Option<String>

}

#[derive(Debug,serde::Deserialize)]
pub struct Login{
    pub username:String,
    pub password:String,
}