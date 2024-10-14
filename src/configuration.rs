use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings, //数据库连接参数
    pub application_port: u16,      //应用程序监听端口
}
#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,      //数据库用户名
    pub password: String,      //数据库密码
    pub port: u16,             //数据库连接端口
    pub host: String,          //主机名
    pub database_name: String, //数据库名字
}
impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgress://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
//读取配置文件
pub fn get_configuration() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::new("configuration.yaml", FileFormat::Yaml))
        .build()?;
    settings.try_deserialize::<Settings>()
}
