#![feature(try_blocks)]
mod event;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, OnceLock};
use std::{fs, thread};

use event::*;
use kissabot::api::*;
use kissabot::event::*;
use kissabot::resources::*;
use kissabot::topic::kokoro::result::{self, anyhow};
use kissabot::topic::{adapter::Adapter, prelude::*};
use serde::{Deserialize, Serialize};
use tungstenite;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;
use url::Url;

#[derive(Deserialize, Serialize)]
struct Config {
    host: String,
    port: u16,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            host: "127.0.0.1".to_string(),
            port: 10240,
        }
    }
}
#[derive(Clone)]
struct OneBot {
    self_id: Arc<OnceLock<u64>>,
    socket: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>,
}
impl Plugin for OneBot {
    type Pars = Event;
    type Global = Kissa;
    fn load(ctx: Context<Self>) -> Result<()> {
        let ctx_ = ctx.clone();
        thread::spawn(move || loop {
            let result: Result<()> = try {
                let message = ctx_
                    .socket
                    .lock()
                    .map_err(|_| anyhow!("获取 socket 失败"))?
                    .read();
                match message {
                    Ok(message) if message.is_text() => {
                        let value = serde_json::from_str::<OneEvent>(message.to_text()?)?;
                        match value {
                            OneEvent {
                                post_type: OneType::Message { content },
                                self_id,
                                time,
                            } => {
                                let sevent = content.try_into_sevent(time, self_id)?;
                                let sevent = SEvent {
                                    content: sevent,
                                    from_adapter: *(ctx_
                                        .self_id
                                        .get()
                                        .ok_or(anyhow!("适配器未初始化"))?),
                                };
                                ctx_.global.publish(sevent)?;
                                result::Ok(())
                            }
                            _ => Ok(()),
                        }?
                    }
                    Ok(message) => {}
                    Err(err) => error!("读取消息出错: {}", err),
                }
            };
            if let Err(err) = result {
                error!("转换事件错误: {}", err)
            }
        });
        let id = add_adapter(&ctx, (&*ctx).clone());
        ctx.self_id
            .set(id)
            .map_err(|err| anyhow!("适配器已存在 ID: {}", err))?;
        info!("OneBot11 适配器加载成功: {}", id);
        Ok(())
    }
}
impl OneBot {
    fn new() -> Result<Self> {
        let filename = "./onebot11.toml";
        let config = match fs::File::open(filename) {
            Ok(mut file) => {
                let mut config = String::new();
                file.read_to_string(&mut config)?;
                toml::from_str(config.as_str())?
            }
            Err(_) => {
                let mut file = fs::File::create(filename)?;
                let config = Config::default();
                file.write_all(&toml::to_string_pretty(&config)?.as_bytes())?;
                config
            }
        };
        let url = Url::parse(format!("ws://{}:{}", config.host, config.port).as_str())?;
        let (mut socket, response) = tungstenite::connect(url)?;
        info!("OneBot11 连接成功: {}", response.status());
        Ok(Self {
            self_id: Arc::new(OnceLock::new()),
            socket: Arc::new(Mutex::new(socket)),
        })
    }
}

impl Adapter for OneBot {}
impl ChannelAPI for OneBot {
    fn get(&self, channel_id: String) -> Result<Channel> {
        todo!()
    }
    fn list(&self, guild_id: String, next: Option<String>) -> Result<Page<Channel>> {
        todo!()
    }
    fn create(&self, guild_id: String, data: Channel) -> Result<Channel> {
        todo!()
    }
    fn update(&self, channel_id: String, data: Channel) -> Result<()> {
        todo!()
    }
    fn delete(&self, channel_id: String) -> Result<()> {
        todo!()
    }
    fn mute(&self, channel_id: String, duration: u64) -> Result<()> {
        todo!()
    }
    fn user_channel_create(&self, user_id: String, guild_id: Option<String>) -> Result<Channel> {
        todo!()
    }
}
impl GuildAPI for OneBot {
    fn get(&self, guild_id: String) -> Result<Guild> {
        todo!()
    }
    fn list(&self, next: Option<String>) -> Result<Page<Guild>> {
        todo!()
    }
    fn approve(&self, message_id: String, approve: bool, comment: String) -> Result<()> {
        todo!()
    }
}
impl GuildRoleAPI for OneBot {
    fn set(&self, guild_id: String, user_id: String, role_id: String) -> Result<()> {
        todo!()
    }
    fn unset(&self, guild_id: String, user_id: String, role_id: String) -> Result<()> {
        todo!()
    }
    fn list(&self, guild_id: String, next: Option<String>) -> Result<Page<GuildRole>> {
        todo!()
    }
    fn create(&self, guild_id: String, role: GuildRole) -> Result<GuildRole> {
        todo!()
    }
    fn update(&self, guild_id: String, role_id: String, role: GuildRole) -> Result<()> {
        todo!()
    }
    fn delete(&self, guild_id: String, role_id: String) -> Result<()> {
        todo!()
    }
}
impl GuildMemberAPI for OneBot {
    fn get(&self, guild_id: String, user_id: String) -> Result<GuildMember> {
        todo!()
    }
    fn list(&self, guild_id: String, next: Option<String>) -> Result<Page<GuildMember>> {
        todo!()
    }
    fn kick(&self, guild_id: String, user_id: String, permanent: Option<bool>) -> Result<()> {
        todo!()
    }
    fn mute(&self, guild_id: String, user_id: String, duration: u64) -> Result<()> {
        todo!()
    }
    fn approve(&self, message_id: String, approve: bool, comment: Option<String>) -> Result<()> {
        todo!()
    }
}
impl LoginAPI for OneBot {
    fn get(&self) -> Result<Login> {
        todo!()
    }
}
impl MessageAPI for OneBot {
    fn create(&self, channel_id: String, content: String) -> Result<Vec<Message>> {
        todo!()
    }
    fn get(&self, channel_id: String, message_id: String) -> Result<Message> {
        todo!()
    }
    fn delete(&self, channel_id: String, message_id: String) -> Result<()> {
        todo!()
    }
    fn update(&self, channel_id: String, message_id: String, content: String) -> Result<()> {
        todo!()
    }
    fn list(&self, channel_id: String, next: Option<String>) -> Result<Page<Message>> {
        todo!()
    }
}
impl ReactionAPI for OneBot {
    fn create(&self, channel_id: String, message_id: String, emoji: String) -> Result<()> {
        todo!()
    }
    fn delete(
        &self,
        channel_id: String,
        message_id: String,
        emoji: String,
        user_id: Option<String>,
    ) -> Result<()> {
        todo!()
    }
    fn clear(&self, channel_id: String, message_id: String, emoji: Option<String>) -> Result<()> {
        todo!()
    }
    fn list(
        &self,
        channel_id: String,
        message_id: String,
        emoji: String,
        next: Option<String>,
    ) -> Result<Page<User>> {
        todo!()
    }
}
impl UserAPI for OneBot {
    fn get(&self, user_id: String) -> Result<User> {
        todo!()
    }
    fn list(&self, next: Option<String>) -> Result<Page<User>> {
        todo!()
    }
    fn approve(&self, message_id: String, approve: bool, comment: Option<String>) -> Result<()> {
        todo!()
    }
}

export_plugin!(OneBot, OneBot::new());
