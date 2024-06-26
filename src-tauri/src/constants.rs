pub mod url {
    //首页列表接口
    pub const INDEX_PAGE_LIST: &str = "https://tieba.baidu.com/f/index/feedlist";

    //首页今日热点接口
    pub const INDEX_TODAY_HOT: &str = "https://tieba.baidu.com/f/index/feedlist";
    //用户头像地址
    pub const USER_AVATAR: &str = "https://gss0.bdstatic.com/6LZ1dD3d1sgCo2Kml5_Y_D3/sys/portrait/item/";
    //Ztion
    pub const ZTION_HOME: &str = "https://ztion.cn";
    //榜单
    pub const TOPIC: &str = "https://tieba.baidu.com/hottopic/browse/topicList";
    //热门吧
    pub const HOT_FORUM: &str = "https://tieba.baidu.com/f/index/rcmdForum?pn=1&rn=12";
    //用户信息，取个id和名字
    pub const USER_INFO: &str = "https://tieba.baidu.com/f/user/json_userinfo?_=1715439890726";
    //帖子详情页
    pub const FEED_PAGE: &str = "https://tieba.baidu.com/p/{pid}";
    //帖子评论区
    pub const FEED_COMMENT_PAGE: &str = "https://tieba.baidu.com/p/{pid}?pn={page}";
}

pub mod header {
    use lazy_static::lazy_static;
    use reqwest::header::HeaderMap;

    lazy_static! {
      pub  static ref COMMON_HEADER: HeaderMap = {
        let mut headers = HeaderMap::new();
        // headers.insert("Accept","text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
        // headers.insert("Host","tieba.baidu.com".parse().unwrap());
        headers.insert("Sec-Ch-Ua","\"Google Chrome\";v=\"125\", \"Chromium\";v=\"125\", \"Not.A/Brand\";v=\"24\"".parse().unwrap());
        headers.insert("Sec-Ch-Ua-Mobile","?0".parse().unwrap());
        headers.insert("Sec-Ch-Ua-Platform","\"Windows\"".parse().unwrap());
        headers.insert("User-Agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse().unwrap());

        headers.insert("Accept-Language","zh-CN,zh;q=0.9".parse().unwrap());
        headers.insert("Accept-Encoding","gzip, deflate, br, zstd".parse().unwrap());
        headers
    };
   }
}

pub mod client {
    use lazy_static::lazy_static;
    use reqwest::{blocking::Client, Proxy};

    lazy_static! {
        pub  static ref CLIENT: Client = {
            Client::builder()
            .proxy(Proxy::http("http://127.0.0.1:7890").unwrap())
            .proxy(Proxy::https("http://127.0.0.1:7890").unwrap())
            .gzip(true)
            .build()
            .unwrap()
        };
    }
}

pub mod app {
    use std::sync::Mutex;

    use lazy_static::lazy_static;

    use crate::config::Config;

    lazy_static! {
        pub  static  ref  CONFIG: Mutex<Config> = {
            Mutex::new(Config::read())
        };
    }
}