use serde::{Deserialize, Serialize};

/// User Command for joining a room.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JoinRoomCommand {
    // The room to join.
    #[serde(rename = "r")]
    pub room: String,
}

/// User Command for leaving a room.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeaveRoomCommand {
    // The room to leave.
    #[serde(rename = "r")]
    pub room: String,
}

/// User Command for sending a message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendMessageCommand {
    // The room to send the message to.
    #[serde(rename = "r")]
    pub room: String,
    // The content of the message.
    #[serde(rename = "c")]
    pub content: String,
}

/// User Command for quitting the whole chat session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuitCommand;

/// A user command which can be sent to the server by a single user session.
/// All commands are processed in the context of the chat server paired with an individual user session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "_ct", rename_all = "snake_case")]
pub enum UserCommand {
    JoinRoom(JoinRoomCommand),
    LeaveRoom(LeaveRoomCommand),
    SendMessage(SendMessageCommand),
    Quit(QuitCommand),
}