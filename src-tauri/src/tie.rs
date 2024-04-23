use std::ops::Add;

use serde::{Deserialize, Serialize};
use url::Url;
use visdom::Vis;

use crate::action;
use crate::constants::url::*;
use crate::util::user::get_user_avatar;

///接口总返回体
#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub no: usize,
    pub error: String,
    pub data: T,
}

///帖子列表返回
#[derive(Serialize, Deserialize)]
pub struct FeedResponseData {
    pub total: usize,
    pub has_more: usize,
    pub html: String,
    pub last_tid: usize,
}

///帖子
#[derive(Serialize, Deserialize, Debug)]
pub struct Feed {
    pub id: String,
    pub forum: String,
    pub title: String,
    pub username: String,
    pub user_avatar: String,
    pub content: String,
    pub img: Vec<String>,
}

///热榜
#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    pub topic_id: usize,
    pub topic_name: String,
    pub topic_desc: String,
    pub topic_avatar: String,
    pub discuss_num: usize,
}
///吧信息
#[derive(Serialize, Deserialize, Debug)]
pub struct Forum {
    pub avatar: String,
    pub forum_id: usize,
    pub forum_name: String,
    pub member_count: usize,
    pub thread_count: usize,
}
impl Feed {
    pub fn from_html(html: &String) -> Vec<Self> {
        let root = Vis::load(html).unwrap();
        let mut li = root.find(".j_feed_li");
        let mut feeds = li.map(|index, e| {
            let c = e.children();
            let id = match e.get_attribute("data-thread-id") {
                None => "1".to_string(),
                Some(attr) => attr.to_string(),
            };
            let forum = c.find(".feed-forum-link").text();
            let title = c.find(".feed-item-link").text();
            let content = c.find(".n_txt").text();
            let img: Vec<String> = c.find(".n_img img").map(|i, img| {
                match img.get_attribute("original") {
                    None => "".to_string(),
                    Some(attr) => action::down_base64(attr.to_string()).unwrap()
                }
            });
            let author = c.find(".post_author");
            let mut user_id = "".to_string();
            if let Some(attr) = author.attr("href") {
                user_id = attr.to_string()
            }
            if user_id.len() > 0 {
                let url = Url::parse(ZTION_HOME.clone().to_string().add(user_id.as_str()).as_str()).unwrap();
                let query_pairs = url.query_pairs().into_owned();
                for (key, value) in query_pairs {
                    if key.eq("id") {
                        user_id = value;
                    }
                }
            }
            let username = author.text();
            Feed {
                id,
                forum,
                title,
                username,
                user_avatar: get_user_avatar(user_id),
                content,
                img,
            }
        });

        let filtered: Vec<Feed> = feeds.drain(..feeds.len()).filter(|f| !f.id.eq("1")).collect();

        filtered
    }
}

