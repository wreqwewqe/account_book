use std::time::Duration;

use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    routing::get,
    response::{IntoResponse, Response},
    Router,
};
use futures::{StreamExt, SinkExt};
use axum::extract::State;
// use futures_util::{stream::{StreamExt}, SinkExt};
use crate::AppState;
// pub async fn handler(ws: WebSocketUpgrade) -> Response {
//     println!("我");
//     ws.on_upgrade(handle_socket)
// }

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<AppState>
) -> impl IntoResponse {
    println!("连接进来了");
    ws.on_upgrade(|socket| handle_socket(socket,app_state))
}
async fn handle_socket(mut socket: WebSocket,app_state:AppState) {
    let (mut writer,mut reader)=socket.split();
    
    let mut username="".to_string();
    let mut channel_receiver=app_state.tx.subscribe();
    //从消息队列拿到消息，发送到客户端
    //相对于tokio的消息队列(收到消息)
    let mut recv_task=tokio::spawn(async move{
        while let Ok(msg)=channel_receiver.recv().await{
            if writer.send(Message::Text(msg)).await.is_err(){
                return 
            };
        }
    });
    //从stream拿到消息，写入到消息队列
    //相对于tokio的消息队列
    let mut channel_sender=app_state.tx.clone();
    let mut send_task=tokio::spawn(async move{
        let mut first=true;
        while let Some(Ok(Message::Text(msg)))=reader.next().await{
            println!("等待读取");
            if first{
                first=false;
                username.push_str(msg.as_str());
                if channel_sender.send(format!("欢迎{}加入聊天室",username)).is_err(){
                    println!("hhhhh");
                    break;
                }
            }else{
                if channel_sender.send(msg).is_err(){
                    println!("结束");
                    break;
                }
            }
            
        }
    });

    tokio::select! {
        _ = (&mut send_task) => {println!("send_task结束"); recv_task.abort()},
        _ = (&mut recv_task) => {println!("recv_task结束"); send_task.abort()},
    }
    println!("共享完成");
    // sender.send(Message::Text("hello".to_string())).await;
} 