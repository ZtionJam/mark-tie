pub mod vo {}

pub mod dto {
    use serde::{Deserialize, Serialize};

    ///Cookie校验和保存
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CookieCheck {
        pub cookie: String,
        pub save: bool,
    }
}

pub mod ex {
    use serde::{Deserialize, Serialize};
    

    use crate::{constants, util};

    ///app配置
    #[derive(Serialize, Deserialize, Debug)]
    pub struct AppConfig {
        pub cookie: String,
    }

    impl AppConfig {
        pub fn read() -> AppConfig {
            serde_json::from_str(&*util::file::read_file_string(&constants::app::CONFIG_PATH)).expect("Error read config")
        }
        fn save(&self) {
            util::file::save_file(&constants::app::CONFIG_PATH, serde_json::to_string(self).unwrap());
        }
    }
}