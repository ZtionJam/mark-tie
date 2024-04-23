pub mod html {
    ///处理一下百度返回的html文档
    /// 替换多余的转义符
    /// 删除首尾的引号
    pub fn format_html(html: &String) -> String {
        let mut new_html = html.clone().to_string();
        new_html = new_html.replace("\\\"", "\"");
        // new_html = new_html.get(1..new_html.len() - 1).unwrap().to_string();

        new_html
    }
}

pub mod user {
    use std::ops::Add;
    use std::time::{SystemTime, UNIX_EPOCH};
    use crate::action;
    use crate::constants::url;

    pub fn get_user_avatar(user_id: String) -> String {
        action::down_base64(url::USER_AVATAR.clone().to_string()
            .add(&*user_id)
            .add("?t=")
            .add(get_unix_timestamp(SystemTime::now()).to_string().as_str())).unwrap()
    }

    fn get_unix_timestamp(time: SystemTime) -> u64 {
        time.duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}