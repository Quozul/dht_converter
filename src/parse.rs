use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct User {
    /// Discord handle
    pub(crate) name: String,
    pub(crate) tag: String,
    pub(crate) avatar: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Server {
    /// For DM, it is the display name of a user
    pub(crate) name: String,
    /// The type of the channel, can be "DM", "GROUP", ...
    #[serde(rename = "type")]
    pub(crate) channel_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Channel {
    /// Index of the server in the `meta.servers` map
    pub(crate) server: u64,
    /// For DM, it is the display name of a user
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Message {
    #[serde(rename = "u")]
    pub(crate) user_index: u64,
    #[serde(rename = "t")]
    pub(crate) timestamp: u64,
    #[serde(rename = "m")]
    pub(crate) message_content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Meta {
    /// User id mapped to the user
    pub(crate) users: HashMap<String, User>,
    /// Essentially `users.keys().map(|id| id.to_string()).collect()`
    #[serde(rename = "userindex")]
    pub(crate) user_index: Vec<String>,
    pub(crate) servers: Vec<Server>,
    /// Channel id mapped to the channel
    pub(crate) channels: HashMap<String, Channel>,
}

pub(crate) type ChannelMessages = HashMap<String, Message>;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Data {
    #[serde(flatten)]
    pub(crate) channel_messages: HashMap<String, ChannelMessages>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DHT {
    pub(crate) meta: Meta,
    pub(crate) data: Data,
}
