use std::{ops::RemAssign, fs::File, io::{BufWriter, Write, BufReader}};

use axum::{extract::{self, State, Multipart, path, Path}, response::IntoResponse, Json};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde_json::json;
use std::io::Read;
use crate::{config::AppError, methods::{get_connection, now}, models::users::{CreateUser, Login, User, CurrentUserInfo, UpdateUser}, schema::users::{self, username, password}};
use diesel::prelude::*;
use uuid::Uuid;
use crate::Pool;
use diesel_async::{
    RunQueryDsl,
};
#[derive(Debug,serde::Deserialize,serde::Serialize)]
pub struct Claims{
    pub uuid:String,
    pub username:String,
    pub password:String,
    pub exp:usize,
}
pub async fn create_user(State(pool): State<Pool>,Json(mut info):Json<CreateUser>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&pool).await?;
    info.uuid=Some(Uuid::new_v4().to_string());
    info.create_at=Some(now());
    let rows=diesel::insert_into(users::table)
                .values(info)
                .execute(&mut conn)
                .await
                .map_err(|e| AppError::err(500,e.to_string()))?
                ;
    Ok(Json(json!({
        "code":200,
        "msg":"创建成功"
    })))
}

pub async fn login(State(pool): State<Pool>,Json(info):Json<Login>)->Result<impl IntoResponse,AppError>{
    println!("我收到了login请求");
    let mut conn=get_connection(&pool).await?;
    let user=users::table.filter(username.eq(info.username).and(password.eq(info.password)))
            .load::<User>(&mut conn)
            .await
            .map_err(|e| AppError::err(500,e.to_string()))?;
    if user.len()<1{
        
        Err(AppError::err(500,"账号或密码错误".to_string()))
    }else{
        println!("我马上返回了");
        let claims=Claims{
            uuid:user[0].uuid.clone(),
            username:user[0].username.clone(),
            password:user[0].password.clone(),
            exp:now().parse::<usize>().unwrap()+60*60*24,
        };
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).map_err(|e| AppError::err(500,e.to_string()))?;
        Ok(Json(json!({
            "code":200,
            "msg":"登录成功",
            "token":token,
            "uuid":user[0].uuid,
            "username":user[0].username
        })))
    }
   
}


//获取当前用户的信息
pub async fn currentUserInfo(State(pool):State<Pool>,Json(info):Json<CurrentUserInfo>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&pool).await?;
    println!("连接建立成功");
    let res=users::table
        .filter(users::uuid.eq(info.parent_uuid))
        .first::<User>(&mut conn)
        .await
        .map_err(|e| AppError::err(500,e.to_string()))?;
    Ok(Json(json!({
        "code":200,
        "msg":"请求成功",
        "data":{
            "info":res,
        }
    })))
}

//更新用户信息
pub async fn update_user(State(pool):State<Pool>,Json(info):Json<UpdateUser>)->Result<impl IntoResponse,AppError>{
    let mut conn=get_connection(&pool).await?;
    diesel::update(users::table.filter(users::uuid.eq(info.uuid)))
        .set((username.eq(info.username),password.eq(info.password)))
        .execute(&mut conn)
        .await
        .map_err(|e| AppError::err(500,e.to_string()))?;
    Ok(Json(json!({
        "code":200,
        "msg":"更新成功"
    })))
}

pub async fn upload(mut multipart: Multipart)->Result<impl IntoResponse, AppError> {
    println!("upload");
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("name:{},data.len:{}", name, data.len());
        let mut file = File::create(format!("./src/assets/{}.jpg",name)).expect("sss");
        file.write_all(&data).unwrap();
    }
    Ok(Json(json!({
        "code":200,
        "msg":"更新成功"
    })))
}

pub async fn get_avatar(Path(filename):Path<String>)->Result<impl IntoResponse,AppError>{
    let f=File::open("./src/assets/".to_string()+&filename+".jpg").map_err(|e| AppError::err(500,e.to_string()))?;
    println!("f:{:?}",f);
    let mut reader=BufReader::new(f);
    println!("转换成了reader");
    let mut data=Vec::new();
    println!("转换成了reader1");
    reader.read_to_end(&mut data).map_err(|e| AppError::err(500,e.to_string()))?;
    // println!("data:{:?}",data);

    Ok(Json(json!({
        "code":200,
        "msg":"获取成功",
        "data":{
            "avatar":data
        },
    })))

}

