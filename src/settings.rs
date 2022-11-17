
use config::{Config, ConfigError, File, FileFormat, ConfigBuilder, builder::AsyncState};
//use tokio::fs::File;


// #[derive(Debug, Deserialize, Clone)]
// pub struct Log {
//     pub level: String,
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct Server {
//     pub port: u16,
//     pub url: String,
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct Rule {
//     pub name: String,
//     pub rule_set: Vec<String>,
// }

// #[derive(Debug, Deserialize, Clone)]
// pub struct Settings {
//     pub server: Server,
//     pub rules: Vec<Rule>,
//     pub log: Log,
//     pub env: ENV,
// }

// const CONFIG_FILE_PATH: &str = "./config/Default.toml";
// const CONFIG_FILE_PREFIX: &str = "./config/";

// #[derive(Clone, Debug, Deserialize)]
// pub enum ENV {
//     Development,
//     Testing,
//     Production,
// }

// impl fmt::Display for ENV {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             ENV::Development => write!(f, "Development"),
//             ENV::Testing => write!(f, "Testing"),
//             ENV::Production => write!(f, "Production"),
//         }
//     }
// }

// impl From<&str> for ENV {
//     fn from(env: &str) -> Self {
//         match env {
//             "Testing" => ENV::Testing,
//             "Production" => ENV::Production,
//             _ => ENV::Development,
//         }
//     }
// }

// impl Settings {
//     pub fn new() -> Result<Self, ConfigError> {
//         let mut builder = ConfigBuilder::<AsyncState>::default();
//         builder = builder.set_default("env", "default")?;
//         builder = builder.set_default("port", "8080")?;
//         builder = builder.set_default("host", "127.0.0.1")?;
//         //builder = builder.add_source(File::new("./config/settings", FileFormat::Json));
//         builder = builder.add_async_source(File::new("./config/default.toml", FileFormat::Toml));

//         let x = builder.build_cloned().await
//     }
// }

pub async fn get_config() -> Result<Config, ConfigError>{
    let builder = ConfigBuilder::<AsyncState>::default()
        .set_default("env", "default")?
        .set_default("port", "8080")?
        .set_default("host", "127.0.0.1")?
         //builder = builder.add_source(File::new("../../src/config/settings", FileFormat::Json));
        .add_source(File::new("./src/config/default.toml", FileFormat::Toml));
        return builder.build().await;
    
}
