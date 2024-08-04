use std::fs::File;
use std::io::Read;
use std::thread::sleep;
use std::time;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use reqwest::blocking::multipart::Form;
use reqwest::blocking::multipart;
use rand::Rng;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct Config {
    msg_count: u64,
    roomid: u64,
    max_second: u64,
    min_second: u64,
    sessdata: String,
    csrf: String,
    msg_list: Vec<String>,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut file = File::open("conf.yaml")?;
    let mut contents = String::new();

    // 读取文件内容到字符串
    file.read_to_string(&mut contents)?;
    // 解析 YAML 字符串
    let mut config: Config = serde_yaml::from_str(&contents)?;
    let client = reqwest::blocking::Client::new();

    // 创建一个随机数生成器
    let mut rng = rand::thread_rng();
    if config.min_second <= 3 {
        config.min_second = 3;
    }
    if config.max_second <= 3 {
        config.max_second = 3;
    }

    let random_number: u32 = rng.gen_range(config.min_second..=config.max_second).try_into().unwrap();


    // 创建包含 Cookie 的请求头
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&format!("SESSDATA={sessdata}", sessdata=config.sessdata))?);

    for _i in 0..config.msg_count {
        let secs = time::Duration::from_secs(random_number.into());
        sleep(secs);
        let msg = gen_msg(&config.msg_list);
        let roomid = config.roomid.to_string();
        let csrf = config.csrf.clone();
        let params = body_param(msg, roomid, csrf);
        let res = client.post("https://api.live.bilibili.com/msg/send")
        .headers(headers.clone())
        .multipart(params)
        .send();
        match res {
            Ok(res) => {
                println!("{:?}", res.text());
            }
            Err(err) => {
                eprintln!("Failed to fetch {:?}: {}", err.url(), err);
                // 错误发生时记录错误，然后继续下一个循环
                continue;
            }
        }
    }

    Ok(())
}

fn gen_msg(msg_list: &Vec<String>) -> String {
    let lenth = msg_list.len();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..lenth);

    return msg_list[index].to_string();
}


fn body_param(msg: String, roomid: String, csrf: String) -> Form {
    let form = multipart::Form::new()
    .text("bubble", "5")
    .text("msg", msg)
    .text("color", "16777215")
    .text("mode", "4")
    .text("room_type", "0")
    .text("jumpfrom", "82002")
    .text("reply_mid", "0")
    .text("reply_attr", "0")
    .text("replay_dmid", "")
    .text("statistics", r#"{"appId":100,"platform":5}"#)
    .text("fontsize", "25")
    .text("rnd", "1722755056")
    .text("roomid", roomid)
    .text("csrf", csrf.clone())
    .text("csrf_token", csrf);

    form
}