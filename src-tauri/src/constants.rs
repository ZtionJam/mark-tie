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
}

pub mod header {
    use lazy_static::lazy_static;
    use reqwest::header::HeaderMap;

    lazy_static! {
      pub  static ref COMMON_HEADER: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert("Accept-Language","zh-CN,zh;q=0.9".parse().unwrap());
        headers.insert("Accept-Encoding","gzip, deflate, br, zstd".parse().unwrap());
        headers
    };
   }
}

pub mod client {
    use lazy_static::lazy_static;
    use reqwest::blocking::Client;

    lazy_static! {
        pub  static ref CLIENT: Client = {
            Client::new()
        };
    }
}