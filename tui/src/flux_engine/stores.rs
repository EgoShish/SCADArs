use std::collections::{HashMap, HashSet};

use circular_queue::CircularQueue;
//use comms::event;

#[derive(Debug, Clone)]
pub enum MessageBoxItem {
    Message { user_id: String, content: String },
    Notification(String),
}

const MAX_MESSAGES_TO_STORE_PER_ROOM: usize = 100;

/// Store_Room это модель комнаты чата

#[derive(Debug, Clone)]
pub struct Store_Room {
    /// наименование комнаты
    pub name: String,
    /// описание комнаты
    pub description: String,
    /// список участников комнаты
    pub users: HashSet<String>,
    /// история сообщений
    pub messages: CircularQueue<MessageBoxItem>,
    /// статус "подключен к комнате"
    pub has_joined: bool,
    /// статус "есть непрочитанное сообщение"
    pub has_unread: bool,
}

impl Default for Store_Room {
    fn default() -> Self {
        Store_Room {
            name: String::new(),
            description: String::new(),
            users: HashSet::new(),
            messages: CircularQueue::with_capacity(MAX_MESSAGES_TO_STORE_PER_ROOM),
            has_joined: false,
            has_unread: false,
        }
    }
}

impl Store_Room {
    pub fn new(name: String, description: String) -> Self {
        Store_Room {
            name,
            description,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum ServerConnectionStatus {
    Uninitalized,
    Connecting,
    Connected { addr: String },
    Errored { err: String },
}

/// Store_Server содержит состояние приложения
#[derive(Debug, Clone)]
pub struct Store_State {
    pub server_connection_status: ServerConnectionStatus,
    /// Currently active room
    pub active_room: Option<String>,
    /// The id of the user
    pub user_id: String,
    /// Storage of room data
    pub room_data_map: HashMap<String, RoomData>,
    /// Timer since app was opened
    pub timer: usize,
}

impl Default for State {
    fn default() -> Self {
        State {
            server_connection_status: ServerConnectionStatus::Uninitalized,
            active_room: None,
            user_id: String::new(),
            room_data_map: HashMap::new(),
            timer: 0,
        }
    }
}
