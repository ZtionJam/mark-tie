use base64::encode;
use serde_json::Value;

use crate::constants::*;
use crate::tie::*;
use crate::util::html::*;

///获取首页分页数据
#[tauri::command]
pub fn get_index_page(limit: &str, offset: &str, lastTid: &str) -> Result<Vec<Feed>, String> {
    let params = [("is_new", "1"), ("tag_id", "like"), ("limit", limit), ("offset", offset), ("last_tid", lastTid)];

    let body_text = &client::CLIENT.get(url::INDEX_PAGE_LIST)
        .headers(header::COMMON_HEADER.clone())
        .query(&params)
        .send().unwrap()
        .text().unwrap();

    let mut res: Response<FeedResponseData> = match serde_json::from_str(&*body_text) {
        Ok(r) => { r }
        Err(_) => {
            println!("获取数据失败{}", body_text);
            return Err("获取数据失败，请重试".to_string());
        }
    };

    if !"sucess".eq(&res.error) {
        return Err("获取数据失败，请重试".to_string());
    }
    res.data.html = format_html(&res.data.html);
    let feeds = Feed::from_html(&res.data.html);
    Ok(feeds)
}

///HOT
#[tauri::command]
pub fn get_topic() -> Result<Vec<Topic>, String> {
    let body_text = &client::CLIENT.get(url::TOPIC)
        .headers(header::COMMON_HEADER.clone())
        .send().unwrap()
        .text().unwrap();
    let mut jo: Value = serde_json::from_str(&*body_text).unwrap();

    if let Some(value) = jo.get("errmsg") {
        if "success".eq(value.as_str().unwrap()) {
            let topic_list = jo.get("data").unwrap().get("bang_topic").unwrap().get("topic_list").unwrap();
            let topics: Vec<Topic> = topic_list.as_array().unwrap().iter().map(|v| {
                let mut t: Topic = serde_json::from_str(v.to_string().as_str()).unwrap();
                t
            }).collect();
            return Ok(topics);
        }
    }

    Err("".to_string())
}

///下载文件Base64
#[tauri::command]
pub fn down_base64(url: String) -> Result<String, String> {
    let response = match client::CLIENT.get(url)
        .headers(header::COMMON_HEADER.clone())
        .send() {
        Ok(response) => { response }
        Err(_) => { return Ok("下载文件失败".to_string()); }
    };

    if response.status().is_success() {
        let bytes = response.bytes().unwrap();
        let content = bytes.as_ref().to_vec();
        let mut base64_str = "data:image/png;base64,".to_string();
        base64_str.push_str(encode(&content).as_str());
        return Ok(base64_str);
    }

    Ok("下载文件失败".to_string())
}

///下载文件Base64
#[tauri::command]
pub fn get_hot_forum() -> Result<Vec<Forum>, String> {
    //只要30条
    let params = [("pn", "1"), ("rn", "30")];

    let body_text = &client::CLIENT.get(url::HOT_FORUM)
        .headers(header::COMMON_HEADER.clone())
        .query(&params)
        .send().unwrap()
        .text().unwrap();

    let mut jo: Value = serde_json::from_str(&*body_text).unwrap();
    if let Some(value) = jo.get("errmsg") {
        if "success".eq(value.as_str().unwrap()) {
            let forum_info = jo.get("data").unwrap().get("forum_info").unwrap();
            let forums: Vec<Forum> = forum_info.as_array().unwrap().iter().map(|v| {
                let mut f: Forum = serde_json::from_str(v.to_string().as_str()).unwrap();
                f.avatar = down_base64(f.avatar).unwrap();
                f
            }).collect();
            return Ok(forums);
        }
    }

    Err("加载热门吧失败".to_string())
}