mod util;
use rand::Rng;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    let prob_upd = 0.8;
    let init_probe = 0.999;

    thread::sleep(Duration::from_secs(5)); //延时5s启动
    let ws_url = env::var("WS").unwrap();
    // let ws_url = "ws://10.243.184.136:30010"; //"ws://10.243.184.136:30010";

    let (socket_send_tx, message_out) = util::create_socket_channel(&ws_url);

    let message_out_fudu = message_out.clone();
    let socket_send_tx_fudu = socket_send_tx.clone();

    let fudu_loop = thread::spawn(move || {
        // 复读模块
        let mut rng = rand::thread_rng();
        let mut group_message_cache: HashMap<u64, (u64, String, f64)> = HashMap::new(); //群id：（成员id，消息，当前概率）
        loop {
            let raw_message = message_out_fudu.recv().unwrap();
            let v: Value = serde_json::from_str(&raw_message).unwrap();

            if v["message_type"] == "group" {
                let mut current_prob = init_probe;
                let message = v["message"].as_str().unwrap();
                let user_id = v["user_id"].as_u64().unwrap();
                let group_id = v["group_id"].as_u64().unwrap();
                println!("收到群{} 来自{}的消息：{}", group_id, user_id, message);

                if let Some((cashe_userid, cashe_message, probe)) =
                    group_message_cache.get(&group_id)
                {
                    current_prob = *probe;
                    if message == cashe_message {
                        if cashe_userid == &user_id {
                            current_prob *= 0.9;
                        } else {
                            current_prob *= prob_upd;
                        }
                    }
                }
                let rand_seed: f64 = rng.gen(); // generates a float between 0 and 1
                println!("随机值：{}", rand_seed);
                if rand_seed > current_prob {
                    let message_to_send = json!({
                        "action": "send_group_msg",
                        "params": {
                            "group_id": group_id,
                            "message": message.to_string(),
                        },
                        "echo": "123"
                    })
                    .to_string();
                    socket_send_tx_fudu.send(message_to_send).unwrap();
                    current_prob = init_probe;
                }
                group_message_cache.insert(group_id, (user_id, message.to_string(), current_prob));

                for (group_id, (userid, message, probe)) in group_message_cache.iter() {
                    println!(
                        "当前消息缓存 群：{} 成员：{} 消息：{} 复读概率：{}",
                        group_id,
                        userid,
                        message,
                        (1.0 - probe)
                    );
                }
            } else {
                println!("未知类型消息，跳过")
            }
        }
    });

    println!("Waiting for child threads to exit");

    let _ = fudu_loop.join();
    // let _ = test_loop.join();
    println!("Exited");
}
