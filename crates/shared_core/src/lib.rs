pub struct AppConfig{
    pub app_name: String,
    pub version: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: "PisoFocus Admin".to_string(),
            version: "0.1.0".to_string(),
        }
    }
}