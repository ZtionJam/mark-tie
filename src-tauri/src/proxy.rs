use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use rand::Rng;
use urlencoding::decode;

use crate::constants::app::CONFIG;
use crate::constants::{client, header};

///启动一个简易的单线程图片代理服务
/// http://localhost:port/img?url=[UrlEncode(地址)]
pub fn start_proxy_server() {
    let port = get_random_unused_port();
    let addr = "127.0.0.1:".to_string() + &port.to_string();
    let listener = TcpListener::bind(addr).unwrap();

    {
        println!("启动代理服务:[127.0.0.1:{}]", port);
        let mut config = CONFIG.lock().unwrap();
        config.port = port.to_string();
        config.flush();
    }
    println!("qidong");
    loop {
        let (tcp_stream, _) = listener.accept().unwrap();
        handle_request(tcp_stream);
    }
}

fn handle_request(mut tcp_stream: TcpStream) {
    let mut reader = BufReader::new(tcp_stream.try_clone().unwrap());
    let mut line = String::new();
    //只需要第一行
    match reader.read_line(&mut line) {
        Ok(_) => {
            if let Ok(mut img_url) = query_url_param(line, "url".to_string()) {
                img_url = decode(&img_url).unwrap();

                if let Ok(data) = down_file(img_url) {
                    let response = "HTTP/1.1 200 OK\r\n\r\n";
                    let _ = tcp_stream.write_all(response.as_bytes());
                    let _ = tcp_stream.write(&data);
                    return;
                }
            }
        }
        Err(e) => println!("Failed to read from connection: {}", e),
    }
    let response = "HTTP/1.1 404 OK\r\n\r\nparam url is not found";
    let _ = tcp_stream.write_all(response.as_bytes());
}

///下载文件
pub fn down_file(url: String) -> Result<Vec<u8>, String> {
    let response = match client::CLIENT
        .get(url)
        .headers(header::COMMON_HEADER.clone())
        .send()
    {
        Ok(response) => response,
        Err(_) => {
            return Err("下载文件失败".to_string());
        }
    };

    if response.status().is_success() {
        let bytes = response.bytes().unwrap();
        return Ok(bytes.as_ref().to_vec());
    }

    Err("下载文件失败".to_string())
}

///从url中分割参数
fn query_url_param(url: String, param: String) -> Result<String, String> {
    if let Some(query_start) = url.find("?") {
        let end_of_url = url.find("HTTP/").unwrap_or(url.len());
        let query_str = &url[query_start + 1..end_of_url - 1];
        let pairs = query_str.split("&");

        // 迭代键值对
        for pair in pairs {
            let mut kv = pair.split("=");
            if let Some(key) = kv.next() {
                if key.eq(param.as_str()) {
                    if let Some(value) = kv.next() {
                        return Ok(value.to_string());
                    }
                }
            }
        }
    }
    Err("none".to_string())
}

///随机获取一个未使用的端口号
fn get_random_unused_port() -> u16 {
    let mut rng = rand::thread_rng();
    let mut port: u16;

    loop {
        // 生成一个随机的端口号
        port = rng.gen_range(1024..65535);

        // 尝试监听端口，如果成功则表示端口未被占用
        if TcpListener::bind(("127.0.0.1", port)).is_ok() {
            break;
        }
    }

    port
}
