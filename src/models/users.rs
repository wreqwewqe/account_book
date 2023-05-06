use diesel::prelude::*;


use crate::schema::users;
#[derive(Debug,serde::Deserialize,Queryable,serde::Serialize)]
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

//根据uuid获取当前用户信息
#[derive(Debug,serde::Deserialize)]
pub struct CurrentUserInfo{
    pub parent_uuid:String
}

#[derive(Debug,serde::Deserialize)]
pub struct UpdateUser{
    pub uuid:String,
    pub username:String,
    pub password:String
}