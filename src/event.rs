use kissabot::event::*;
use kissabot::resources::*;
use kissabot::topic::kokoro::result::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait IntoSEvent {
    fn try_into_sevent(self, one_event: &OneEvent) -> Result<SEventContent>;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OneEvent {
    pub time: u64,
    pub self_id: u64,
    #[serde(flatten)]
    pub post_type: OneType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "post_type")]
pub enum OneType {
    #[serde(rename = "message")]
    Message {
        #[serde(flatten)]
        content: OneMessageType,
    },
    #[serde(rename = "notice")]
    Notice {
        #[serde(flatten)]
        content: OneNotice,
    },
    #[serde(rename = "request")]
    Request {
        #[serde(flatten)]
        content: OneRequest,
    },
    #[serde(rename = "meta_event")]
    MetaEvent {
        #[serde(flatten)]
        content: OneMetaEvent,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "message_type")]
pub enum OneMessageType {
    #[serde(rename = "private")]
    Private {
        sub_type: Option<OneSubType>,
        message_id: u32,
        user_id: u64,
        message: Value,
        raw_message: String,
        font: u32,
        sender: OnePrivateSender,
    },
}

impl IntoSEvent for OneMessageType {
    fn try_into_sevent(self, one_event: &OneEvent) -> Result<SEventContent> {
        match self {
            OneMessageType::Private {
                sub_type: _,
                message_id,
                user_id,
                message: _,
                raw_message,
                font: _,
                sender,
            } => {
                let channel = Channel {
                    id: format!("private-{}", user_id),
                    ty: ChannelType::Text,
                    name: Some(sender.nickname.clone()),
                    parent_id: None,
                };
                let user = User {
                    id: user_id.to_string(),
                    name: Some(sender.nickname.clone()),
                    nick: Some(sender.nickname.clone()),
                    avatar: None,
                    is_bot: None,
                };
                Ok(SEventContent {
                    id: message_id as u64,
                    ty: SEventType::MessageCreated,
                    platfrom: "onebot11".to_string(),
                    self_id: one_event.self_id.to_string(),
                    timestamp: one_event.time as u128,
                    argv: None,
                    button: None,
                    channel: Some(channel.clone()),
                    guild: None,
                    login: None,
                    member: None,
                    message: Some(Message {
                        id: message_id.to_string(),
                        content: raw_message,
                        channel: Some(channel),
                        guild: None,
                        member: None,
                        user: Some(user.clone()),
                        created_at: Some(one_event.time),
                        updated_at: None,
                    }),
                    operator: Some(user.clone()),
                    role: None,
                    user: Some(user),
                })
            }
            _ => Err(anyhow!("未知消息类型")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnePrivateSender {
    pub user_id: u64,
    pub nickname: String,
    pub sex: OneSex,
    pub age: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OneSex {
    #[serde(rename = "male")]
    Male,
    #[serde(rename = "female")]
    Female,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OneSubType {
    #[serde(rename = "friend")]
    Friend,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "other")]
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OneNotice {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OneRequest {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OneMetaEvent {}
