use axum::response::IntoResponse;
use axum::{Json};
use serde_json::json;
use crate::Pool;
pub struct Share{
    pool:Pool
}

impl Share{
    pub fn new(pool:Pool)->Self{
        Share{pool}
    }
}

#[derive(Debug)]
pub struct AppError{
    pub code:i32,
    pub msg:String
}

impl AppError{
    pub fn err(code:i32,msg:String)->Self{
        AppError{
            code,
            msg
        }
    }
}

impl IntoResponse for AppError{
    fn into_response(self) -> axum::response::Response {
        Json(json!({
            "code":self.code,
            "msg":self.msg
        })).into_response()
    }
}