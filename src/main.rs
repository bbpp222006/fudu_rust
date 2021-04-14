use crossbeam::crossbeam_channel::{bounded, select, Receiver, Sender};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tungstenite::{connect, Message};
use url::Url;
use std::env;





fn main() {
    thread::sleep(Duration::from_secs(5)); //延时5s启动
    let ws_url =env::var("WS").unwrap(); //"ws://10.243.159.138:30010";
    let (mut send_socket, send_response) =
        connect(Url::parse(&format!("{}/api", ws_url)).unwrap()).expect("Can't connect");
    let (mut receive_socket, receive_response) =
        connect(Url::parse(&format!("{}/event", ws_url)).unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!(
        "Response HTTP code: {} {}",
        send_response.status(),
        receive_response.status()
    );

    let (heartbeat_in, heartbeat_out) = bounded(10);
    let (message_in, message_out) = bounded(10);
    let (socket_send_tx, socket_send_rx): (Sender<String>, Receiver<String>) = bounded(10);

    let send_loop = thread::spawn(move || {
        // 发送消息县城
        loop {
            let meesage_send = socket_send_rx.recv().unwrap();
            println!("发送{}", meesage_send);
            send_socket
                .write_message(Message::Text(meesage_send))
                .unwrap();
        }
    });

    let message_loop = thread::spawn(move || {
        // 接收消息县城
        loop {
            let msg = receive_socket
                .read_message()
                .expect("Error reading message");
            let v: Value = serde_json::from_str(&msg.to_string()).unwrap();

            if v["meta_event_type"] == "heartbeat" {
                // println!("心跳包加入通道！");
                heartbeat_in.send(msg.to_string()).unwrap();
            } else {
                // println!("消息加入通道！");
                message_in.send(msg.to_string()).unwrap();
            }
        }
    });

    let heartbeat_loop = thread::spawn(move || {
        // 检测心跳是否正常，不正常则尝试重连
        loop {
            select! {
                recv(heartbeat_out) -> _ =>  {
                    println!("心跳检测成功")
                },
                default(Duration::from_secs(600)) => {
                    panic!("10分钟没有检测到心跳，程序退出")
                },
            }
        }
    });

    let message_out_fudu = message_out.clone();
    let socket_send_tx_fudu = socket_send_tx.clone();
    let fudu_loop = thread::spawn(move || {
        // 复读模块
        let mut group_message_cache = HashMap::new();
        let mut group_userid_cache = HashMap::new();
        loop {
            let raw_message = message_out_fudu.recv().unwrap();
            let v: Value = serde_json::from_str(&raw_message).unwrap();

            // if v["message_type"]=="private"{
            //     let message=v["message"].as_str().unwrap();
            //     let user_id=v["user_id"].as_u64().unwrap();
            //     println!("收到来自{}的私聊消息：{}",user_id,v);

            //     let message_to_send = json!({
            //         "action": "send_private_msg",
            //         "params": {
            //             "user_id": user_id,
            //             "message": message,
            //         },
            //         "echo": "123"
            //     }).to_string();
            //     socket_send_tx_fudu.send(message_to_send).unwrap();

            // }else
            if v["message_type"] == "group" {
                let message = v["message"].as_str().unwrap();
                let user_id = v["user_id"].as_u64().unwrap();
                let group_id = v["group_id"].as_u64().unwrap();
                println!("收到群{} 来自{}的消息：{}", group_id, user_id, message);
                for (contact, number) in group_message_cache.iter() {
                    println!("Calling {}: {}", contact, &number); 
                }

                if let Some(cashe_message) = group_message_cache.get(&group_id){
                    if cashe_message==message{
                        if let Some(cashe_userid) = group_userid_cache.get(&group_id){
                            if &user_id != cashe_userid{
                                let message_to_send = json!({
                                    "action": "send_group_msg",
                                    "params": {
                                        "group_id": group_id,
                                        "message": message.to_string()+"!",
                                    },
                                    "echo": "123"
                                })
                                .to_string();
                                socket_send_tx_fudu.send(message_to_send).unwrap();
                                group_message_cache.remove(&group_id);
                                continue;
                            }else{
                                println!("同一人复读");
                                let message_to_send = json!({
                                    "action": "send_group_msg",
                                    "params": {
                                        "group_id": group_id,
                                        "message": "¿",
                                    },
                                    "echo": "123"
                                })
                                .to_string();
                                socket_send_tx_fudu.send(message_to_send).unwrap();
                                group_message_cache.remove(&group_id);
                                continue;
                            }
                        }
                    }
                }
                group_message_cache.insert(group_id, message.to_string());
                group_userid_cache.insert(group_id, user_id);
                
            } else {
                println!("未知类型消息，跳过")
            }
        }
    });

    println!("Waiting for child threads to exit");

    let _ = message_loop.join();
    let _ = heartbeat_loop.join();
    let _ = send_loop.join();
    let _ = fudu_loop.join();
    // let _ = test_loop.join();
    println!("Exited");
}
