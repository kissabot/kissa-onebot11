use serde::{Deserialize, Serialize};
use serde_json::Value;

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
        sub_type: OneSubType,
        message_id: u32,
        user_id: u64,
        message: Value,
        raw_message: String,
        font: u32,
        sender: OnePrivateSender,
    },
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
