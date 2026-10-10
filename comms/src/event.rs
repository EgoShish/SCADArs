use serde::{Deserialize, Serialize};

/// Детали комнаты, которой поделились
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomDetail {
    /// Имя комнаты
    #[serde(rename = "n")]
    pub name: String,
    /// описание комнаты
    #[serde(rename = "d")]
    pub description: String,
}

/// Зарегистрированный пользователь
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginSuccessfulReplyEvent {
    /// ID сессии для подключения
    #[serde(rename = "s")]
    pub session_id: String,
    /// ID зарегистрированного пользователя
    #[serde(rename = "u")]
    pub user_id: String,
    /// The list of rooms the user can participate, unique and ordered
    #[serde(rename = "rs")]
    pub rooms: Vec<RoomDetail>,
}

/// Users new room participation status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomParticipationStatus {
    Joined,
    Left,
}

/// A user has joined or left a room
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomParticipationBroacastEvent {
    /// The slug of the room the user has joined or left
    #[serde(rename = "r")]
    pub room: String,
    /// The id of the user that has joined or left
    #[serde(rename = "u")]
    pub user_id: String,
    /// The new status of the user in the room
    #[serde(rename = "s")]
    pub status: RoomParticipationStatus,
}

/// A reply to the user when they have joined a room
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserJoinedRoomReplyEvent {
    /// The slug of the room the user has joined
    #[serde(rename = "r")]
    pub room: String,
    /// The users currently in the room, unique and ordered
    #[serde(rename = "us")]
    pub users: Vec<String>,
}

/// A user has sent a message to a room
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserMessageBroadcastEvent {
    /// The slug of the room the user has sent the message to
    #[serde(rename = "r")]
    pub room: String,
    /// The id of the user that has sent the message
    #[serde(rename = "u")]
    pub user_id: String,
    /// The content of the message
    #[serde(rename = "c")]
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "_et", rename_all = "snake_case")]
/// Events that can be sent to the client
/// Events maybe related to different users and rooms, the receipient is a single chat session
pub enum Event {
    LoginSuccessful(LoginSuccessfulReplyEvent),
    RoomParticipation(RoomParticipationBroacastEvent),
    UserJoinedRoom(UserJoinedRoomReplyEvent),
    UserMessage(UserMessageBroadcastEvent),
}