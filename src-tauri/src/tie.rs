use std::ops::Add;

use serde::{Deserialize, Serialize};
use url::Url;
use visdom::Vis;

use crate::constants::url::*;
use crate::util::user::get_user_avatar;

///接口总返回体
#[derive(Serialize, Deserialize, Debug)]
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
    pub time: String,
    pub post_num: String,
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

///吧信息
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub user_name_show: String,
    pub mobilephone: String,
    pub open_uid: usize,
    pub user_portrait: String,
}

///帖内信息
#[derive(Serialize, Deserialize, Debug)]
pub struct FeedInfo {
    pub feed_title: String,
    pub feed_content: String,
    pub feed_img_list: Vec<String>,
    pub master: Master,
}

///作者信息
#[derive(Serialize, Deserialize, Debug)]
pub struct Master {
    pub name: String,
    pub avatar: String,
    pub level: String,
}


///评论
#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub pid: String,
    pub content: String,
    pub ip: String,
    pub floor: String,
    pub time: String,
    pub author: bool,
    pub img_list: Vec<String>,
    pub comment_user:Master
}
///评论分页
#[derive(Serialize, Deserialize, Debug)]
pub struct CommentPage {
    pub data: Vec<Comment>,
    pub size: usize,
    pub has_next:bool,
    pub total:usize
}

impl Feed {
    pub fn from_html(html: &String) -> Vec<Self> {
        let root = Vis::load(html).unwrap();
        let li = root.find(".j_feed_li");
        let mut feeds = li.map(|_, e| {
            let c = e.children();
            let id = match e.get_attribute("data-thread-id") {
                None => "1".to_string(),
                Some(attr) => attr.to_string(),
            };
            let forum = c.find(".feed-forum-link").text();
            let title = c.find(".feed-item-link").text();
            let content = c.find(".n_txt").text();
            let img: Vec<String> = c.find(".n_img img").map(|_, img| {
                match img.get_attribute("original") {
                    None => "".to_string(),
                    Some(attr) => attr.to_string()
                }
            });
            let author = c.find(".post_author");
            let mut user_id = "".to_string();
            if let Some(attr) = author.attr("href") {
                user_id = attr.to_string()
            }
            if user_id.len() > 0 {
                let url = Url::parse(ZTION_HOME.to_string().add(user_id.as_str()).as_str()).unwrap();
                let query_pairs = url.query_pairs().into_owned();
                for (key, value) in query_pairs {
                    if key.eq("id") {
                        user_id = value;
                    }
                }
            }
            let username = author.text();
            let time=c.find(".time").text();
            let post_num=c.find(".list-post-num em").text();
            Feed {
                id,
                forum,
                title,
                username,
                user_avatar: get_user_avatar(user_id),
                content,
                img,
                time,
                post_num
            }
        });

        let filtered: Vec<Feed> = feeds.drain(..feeds.len()).filter(|f| !f.id.eq("1")).collect();

        filtered
    }
}

