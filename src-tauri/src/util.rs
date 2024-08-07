pub mod html {
    ///处理一下百度返回的html文档
    /// 替换多余的转义符
    /// 删除首尾的引号
    pub fn format_html(html: &String) -> String {
        let mut new_html = html.clone().to_string();
        new_html = new_html.replace("\\\"", "\"");

        new_html
    }
}

pub mod http {
    use reqwest::header::HeaderMap;

    use crate::constants::app::CONFIG;
    use crate::constants::header;

    pub fn get_now_header() -> HeaderMap {
        let mut header = header::COMMON_HEADER.clone();
        let config = CONFIG.lock().unwrap();
        if config.cookie.len() > 0 {
            header.insert("Cookie", config.cookie.parse().unwrap());
        }
        header
    }
    pub fn get_cookie_header(cookie: &String) -> HeaderMap {
        let mut header = header::COMMON_HEADER.clone();
        header.insert("Cookie", cookie.parse().unwrap());
        header
    }
}

pub mod user {
    use std::ops::Add;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::constants::url;

    pub fn get_user_avatar(user_id: String) -> String {
        url::USER_AVATAR
            .to_string()
            .add(&*user_id)
            .add("?t=")
            .add(get_unix_timestamp(SystemTime::now()).to_string().as_str())
    }

    fn get_unix_timestamp(time: SystemTime) -> u64 {
        time.duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}
pub mod file {
    use std::fs;
    use std::fs::{create_dir_all, OpenOptions};
    use std::io::Write;
    use std::path::PathBuf;

    pub fn save_file(path: &PathBuf, content: String) {
        let _ = create_dir_all(path.parent().unwrap());
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .and_then(|mut file| file.write_all(content.as_bytes()))
            .map(|_| ())
            .expect("Save file error")
    }
    pub fn read_file_string(path: &PathBuf) -> String {
        fs::read_to_string(path).unwrap_or(String::new())
    }
}
