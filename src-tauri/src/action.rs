use serde_json::Value;
use visdom::Vis;

use crate::config::Config;
use crate::constants;
use crate::constants::*;
use crate::constants::app::CONFIG;
use crate::tie::*;
use crate::util::html::*;
use crate::util::http::*;

///获取首页分页数据
#[tauri::command]
pub fn get_index_page(limit: &str, offset: &str, last_tid: &str) -> Result<Vec<Feed>, String> {
    let params = [("is_new", "1"), ("tag_id", "like"), ("limit", limit), ("offset", offset), ("last_tid", last_tid)];

    let body_text = &client::CLIENT.get(url::INDEX_PAGE_LIST)
        .headers(get_now_header())
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
    let jo: Value = serde_json::from_str(&*body_text).unwrap();

    if let Some(value) = jo.get("errmsg") {
        if "success".eq(value.as_str().unwrap()) {
            let topic_list = jo.get("data").unwrap().get("bang_topic").unwrap().get("topic_list").unwrap();
            let topics: Vec<Topic> = topic_list.as_array().unwrap().iter().map(|v| {
                let t: Topic = serde_json::from_str(v.to_string().as_str()).unwrap();
                t
            }).collect();
            return Ok(topics);
        }
    }

    Err("".to_string())
}


///热门贴吧
#[tauri::command]
pub fn get_hot_forum() -> Result<Vec<Forum>, String> {
    //只要30条
    let params = [("pn", "1"), ("rn", "30")];

    let body_text = &client::CLIENT.get(url::HOT_FORUM)
        .headers(get_now_header())
        .query(&params)
        .send().unwrap()
        .text().unwrap();

    let jo: Value = serde_json::from_str(&*body_text).unwrap();
    if let Some(value) = jo.get("errmsg") {
        if "success".eq(value.as_str().unwrap()) {
            let forum_info = jo.get("data").unwrap().get("forum_info").unwrap();
            let forums: Vec<Forum> = forum_info.as_array().unwrap().iter().map(|v| {
                let f: Forum = serde_json::from_str(v.to_string().as_str()).unwrap();
                f
            }).collect();
            return Ok(forums);
        }
    }

    Err("加载热门吧失败".to_string())
}

///获取或者写入cookie
/// cookie有值写入，无值获取
#[tauri::command]
pub fn get_or_set_cookie(cookie: String) -> Result<String, String> {
    let mut config = match constants::app::CONFIG.lock() {
        Ok(c) => { c }
        Err(_) => { return Err("操作失败，请重试".to_string()); }
    };
    if cookie.len() == 0 {
        return Ok(config.cookie.clone());
    }
    config.cookie = cookie;
    config.flush();

    Ok("ok".to_string())
}

///获取用户信息
#[tauri::command]
pub fn get_user_info() -> Result<UserInfo, String> {
    let header = get_now_header();
    let body_text = &client::CLIENT.get(url::USER_INFO)
        .headers(header)
        .send().unwrap()
        .text().unwrap();
    if !"null".eq(body_text) {
        let res: Response<UserInfo> = serde_json::from_str(&*body_text).unwrap();
        return Ok(res.data);
    }


    Err("未登录".to_string())
}

///获取用户信息
#[tauri::command]
pub fn get_config() -> Result<Config, String> {
    let config = CONFIG.lock().unwrap();
    Ok(config.clone())
}


#[tauri::command]
pub fn get_feed_info(pid: String) -> FeedInfo {
    let page_url = url::FEED_PAGE.replace("{pid}", pid.as_str());
    println!("地址{}", page_url);
    let body_text = &client::CLIENT.get(page_url)
        .headers(get_now_header())
        .send().unwrap()
        .text().unwrap();
    let root = Vis::load(body_text).unwrap();
    let title = root.find(".left_section .core_title_txt").text();
    let content = root.find(".left_section .p_postlist > div:nth-child(1) .d_post_content").text();
    let img_list: Vec<String> = root.find(".p_postlist > div:nth-child(1) .d_post_content_main  .d_post_content img").map(|_, e| {
        if let Some(o) = e.get_attribute("src") {
            return o.to_string();
        }
        "".to_string()
    }).iter().filter(|u| u.starts_with("http://tiebapic.baidu.com/forum")).cloned().collect();
    let master_name = root.find(".p_postlist > div:nth-child(1) .d_author .p_author_name").text();
    let master_level = root.find(".p_postlist > div:nth-child(1) .d_author .d_badge_title ").text();
    let master_avatar = match root.find(".p_postlist > div:nth-child(1) .d_author .p_author_face img").attr("src") {
        None => "".to_string(),
        Some(o) => o.to_string()
    };

    FeedInfo {
        feed_title: title,
        feed_content: content,
        feed_img_list: img_list,
        master: Master {
            name: master_name,
            avatar: master_avatar,
            level: master_level,
        },
    }
}
#[tauri::command]
pub fn get_feed_comment(pid: String,page:u32) -> CommentPage {
    let page_url = url::FEED_COMMENT_PAGE.replace("{pid}", pid.as_str()).replace("{page}", page.to_string().as_str());
    let body_text = &client::CLIENT.get(page_url)
        .headers(get_now_header())
        .send().unwrap()
        .text().unwrap();
    let root = Vis::load(body_text).unwrap();
    let post_list=root.find(".left_section .p_postlist > div:not(:first-child).l_post");
    let comments:Vec<Comment>=post_list.map(|_,e|{
       let child= e.children();
       let comment_user=  Master{
            name:child.find(".d_name .p_author_name").text(),
            level:child.find(".p_badge .d_badge_title").text(),
            avatar:match child.find(".p_author_face img").attr("src") {
                Some(att) => {att.to_string()},
                None => {"".to_string()},
            }
        };
        let content=child.find(".p_content .d_post_content").text().trim().to_string();
        let img_list=child.find(".p_content .BDE_Image").map(|_,e|{
           let child= e.children();
           match child.attr("src") {
               Some(att) => {att.to_string()},
               None => {"".to_string()},
           }
        }).into();
        Comment{
            content,
            img_list,
            comment_user
        }
    }).into();

    let total=match root.find(".pb_footer .p_thread .l_reply_num .red:first-child").text().parse:: <usize>() {
        Ok(n) => {n},
        Err(_) => {0},
    };

    CommentPage{
        size:*&comments.len(),
        data:comments,
        total,
        has_next:false
    }
}












